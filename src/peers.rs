use std::net::SocketAddr;
use std::iter::{Iterator, IntoIterator};

use fxhash::{FxHashSet, FxHashMap};

use indexmap::IndexMap;
use crate::node::{Node, PeerNode, Digest};
use crate::utils::{Rng, rand};

pub struct Peers {
  list: IndexMap<String, PeerNode>,
  offset: usize,
  roots: Vec<SocketAddr>,
}

impl Peers {
  pub fn new(roots: Vec<SocketAddr>) -> Self {
    Self {
      list: IndexMap::new(),
      offset: 0,
      roots,
    }
  }

  pub fn len(&self) -> usize { self.list.len() }

  pub fn get(&self, identifier: &str) -> Option<&PeerNode> {
    self.list.get(identifier)
  }

  pub fn get_mut(&mut self, identifier: &str) -> Option<&mut PeerNode> {
    self.list.get_mut(identifier)
  }

  pub fn add(&mut self, node: PeerNode) -> Option<PeerNode> {
    self.list.insert(node.identifier().to_owned(), node)
  }

  pub fn digest(&self) -> Vec<Digest> {
    self.list.iter().map(|(_,n)| n.digest()).collect()
  }

  pub fn prune(&mut self) {
    self.list.retain(|_,n| !n.discardable());
  }

  fn partition(&self) -> (Vec<&PeerNode>, Vec<&PeerNode>) {
    self.list.values().partition(|n| n.active())
  }

  pub fn actives(&self) -> FxHashMap<&str, &PeerNode> {
    let (actives, _) = self.partition();
    return FxHashMap::from_iter(actives.iter().map(|&n| (n.identifier(), n)));
  }

  fn next(&mut self) -> Option<&PeerNode> {
    match self.list.len() {
      0 => { None }
      n => {
        self.offset += 1;
        match self.list.get_index(self.offset % n) {
          Some((_, node)) => { Some(node) }
          None => { None }
        }
      }
    }
  }

  pub fn targets(&mut self, rng: &mut Rng) -> Vec<SocketAddr> {
    let mut sample = FxHashSet::<SocketAddr>::default();

    if self.len() == 0 {
      return self.roots.clone();
    }

    // cycle through all peer nodes
    self.next().and_then(|n| Some(sample.insert(*n.address())));

    // sometimes, add a root
    if !self.roots.is_empty() && rng.rand_float() < 0.2 {
      sample.insert(*rand::choose(rng, &self.roots));
    }

    let (mut actives, inactives) = self.partition();

    // sometimes, add an inactive
    if !inactives.is_empty() && rng.rand_float() < 0.1 {
      sample.insert(*rand::choose(rng, &inactives).address());
    }

    // add random actives to fill
    let count = usize::min(
      actives.len(),
      isize::max(0, 4 - sample.len() as isize) as usize
    );

    rand::shuffle(rng, &mut actives, count);
    for n in &actives[0..count] {
      sample.insert(*n.address());
    }

    return sample.into_iter().collect();
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::utils::rng;
  use crate::utils::testing::{
      addr, addrs, addr_from, advance_clock, create_peer,
  };

  fn active_peer(peers: &mut Peers, id: &str) {
    peers.get_mut(id).unwrap()
      .apply(1, vec![("k".into(), (42.into(), 1))]);
  }

  #[test]
  fn test_peers_creation() {
    let peers = Peers::new(addrs());
    assert_eq!(peers.len(), 0);
    assert!(peers.digest().is_empty());
    assert!(peers.actives().is_empty());
  }

  #[test]
  fn test_peers_add_and_get() {
    let mut peers = Peers::new(addrs());
    peers.add(create_peer(1));
    assert_eq!(peers.len(), 1);
    assert!(peers.get("peer1").is_some());
    assert!(peers.get_mut("peer1").is_some());
    assert_eq!(peers.get("peer1").unwrap().identifier(), "peer1");
  }

  #[test]
  fn test_peers_digest() {
    let mut peers = Peers::new(addrs());
    peers.add(create_peer(1));
    peers.add(create_peer(2));
    assert_eq!(peers.len(), 2);
    assert_eq!(peers.digest(), [
        ("peer1".into(), 0),
        ("peer2".into(), 0),
    ]);
  }

  #[test]
  fn test_peers_prune() {
      let mut peers = Peers::new(addrs());
      peers.add(create_peer(1));
      advance_clock(86400.0); // time passes...
      peers.add(create_peer(2));
      assert_eq!(peers.len(), 2);

      // no peers are discardable
      peers.prune();
      assert_eq!(peers.len(), 2);

      // time passes...
      advance_clock(86400.0);
      peers.prune();
      assert_eq!(peers.len(), 1);
      assert!(peers.get("peer1").is_none());
      assert!(peers.get("peer2").is_some());
  }

  #[test]
  fn test_peers_prune_with_all_discardable() {
      let mut peers = Peers::new(addrs());
      peers.add(create_peer(1));
      peers.add(create_peer(2));
      advance_clock(86400.0 * 2.0); // Make all peers discardable
      peers.prune();
      assert_eq!(peers.len(), 0);
  }

  #[test]
  fn test_peers_actives() {
    let mut peers = Peers::new(addrs());
    peers.add(create_peer(1));
    peers.add(create_peer(2));

    // none are active initially
    assert!(peers.actives().is_empty());

    fn id(peer: Option<&&PeerNode>) -> String {
      peer.unwrap().identifier().to_string()
    }

    active_peer(&mut peers, "peer1");

    let actives = peers.actives();
    assert_eq!(actives.len(), 1);
    assert_eq!(id(actives.get("peer1")), "peer1");

    active_peer(&mut peers, "peer2");

    let actives = peers.actives();
    assert_eq!(actives.len(), 2);
    assert_eq!(id(actives.get("peer1")), "peer1");
    assert_eq!(id(actives.get("peer2")), "peer2");
  }

  #[test]
  fn test_peers_next() {
    let mut peers = Peers::new(addrs());
    peers.add(create_peer(1));
    peers.add(create_peer(2));

    fn next_id(peers: &mut Peers) -> String {
      peers.next().unwrap().identifier().to_string()
    }

    assert_eq!(next_id(&mut peers), "peer2");
    assert_eq!(next_id(&mut peers), "peer1");
    assert_eq!(next_id(&mut peers), "peer2");
    assert_eq!(next_id(&mut peers), "peer1");
    assert_eq!(next_id(&mut peers), "peer2");

    peers.add(create_peer(3));
    assert_eq!(next_id(&mut peers), "peer1");
    assert_eq!(next_id(&mut peers), "peer2");
    assert_eq!(next_id(&mut peers), "peer3");
    assert_eq!(next_id(&mut peers), "peer1");
  }

  #[test]
  fn test_peers_next_empty() {
      let mut peers = Peers::new(addrs());
      assert!(peers.next().is_none());
  }

  #[test]
  fn test_peers_targets_no_peers() {
    let mut rng = rng(None);
    let mut peers = Peers::new(addrs());
    let targets = peers.targets(&mut rng);
    assert_eq!(targets, addrs());
  }

  #[test]
  fn test_peers_targets_includes_next_peer() {
    let mut rng = rng(Some(39));
    let mut peers = Peers::new(addrs());
    peers.add(create_peer(1));
    peers.add(create_peer(2));

    assert_eq!(peers.targets(&mut rng), [
      "127.1.1.22:3322".parse().unwrap(),
    ]);
    assert_eq!(peers.targets(&mut rng), [
      "127.1.1.21:3322".parse().unwrap(),
    ]);
    assert_eq!(peers.targets(&mut rng), [
      "127.1.1.22:3322".parse().unwrap(),
    ]);
  }

  #[test]
  fn test_peers_targets_sometimes_includes_a_root() {
    let mut rng = rng(Some(44));
    let mut peers = Peers::new(addrs());
    peers.add(create_peer(1));

    assert_eq!(peers.targets(&mut rng), [
      "127.1.1.13:3322".parse().unwrap(),
      "127.1.1.21:3322".parse().unwrap(),
    ]);
  }

  #[test]
  fn test_peers_targets_sometimes_includes_an_inactive_peer() {
    let mut rng = rng(Some(58));
    let mut peers = Peers::new(addrs());
    peers.add(create_peer(1));
    peers.add(create_peer(2));

    assert_eq!(peers.targets(&mut rng), [
      "127.1.1.21:3322".parse().unwrap(),
      "127.1.1.22:3322".parse().unwrap(),
    ]);
  }

  #[test]
  fn test_peers_targets_fills_remaining_slots_with_random_actives() {
    let mut peers = Peers::new(addrs());

    for i in 1..10 {
      peers.add(create_peer(i));
      active_peer(&mut peers, &format!("peer{i}"));
    }
    peers.add(create_peer(10));

    assert_eq!(peers.targets(&mut rng(Some(41))), [
      "127.1.1.22:3322".parse().unwrap(),
      "127.1.1.23:3322".parse().unwrap(),
      "127.1.1.24:3322".parse().unwrap(),
      "127.1.1.26:3322".parse().unwrap(),
    ]);

    assert_eq!(peers.targets(&mut rng(Some(44))), [
      "127.1.1.23:3322".parse().unwrap(),
      "127.1.1.13:3322".parse().unwrap(),
      "127.1.1.21:3322".parse().unwrap(),
    ]);

    assert_eq!(peers.targets(&mut rng(Some(58))), [
      "127.1.1.210:3322".parse().unwrap(),
      "127.1.1.24:3322".parse().unwrap(),
      "127.1.1.25:3322".parse().unwrap(),
      "127.1.1.26:3322".parse().unwrap(),
    ]);
  }
}
