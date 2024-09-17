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
    self.offset += 1;
    match self.list.get_index(self.offset % self.list.len()) {
      Some((_, node)) => { Some(node) }
      None => { None }
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
    if rng.rand_float() < 0.1 {
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
  use crate::utils::testing::{addr, addrs, addr_from};

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
    let peer = PeerNode::new("p1".into(), addr());
    peers.add(peer);
    assert_eq!(peers.len(), 1);
    assert!(peers.get("p1").is_some());
    assert!(peers.get_mut("p1").is_some());
    assert_eq!(peers.get("p1").unwrap().identifier(), "p1");
  }

  #[test]
  fn test_peers_digest() {
    let mut peers = Peers::new(addrs());
    let peer1 = PeerNode::new("p1".into(), addr());
    peers.add(peer1);
    let peer2 = PeerNode::new("p2".into(), addr_from("127.1.1.20:3322"));
    peers.add(peer2);
    assert_eq!(peers.len(), 2);
    assert_eq!(peers.digest(), [("p1".into(), 0), ("p2".into(), 0)]);
  }

  #[test]
  fn test_peers_next() {
    let mut peers = Peers::new(addrs());
    peers.add(PeerNode::new("p1".into(), addr()));
    peers.add(PeerNode::new("p2".into(), addr_from("127.1.1.20:3322")));
    assert_eq!(peers.next().unwrap().identifier(), "p2");
    assert_eq!(peers.next().unwrap().identifier(), "p1");
    assert_eq!(peers.next().unwrap().identifier(), "p2");
    assert_eq!(peers.next().unwrap().identifier(), "p1");
    assert_eq!(peers.next().unwrap().identifier(), "p2");
    peers.add(PeerNode::new("p3".into(), addr_from("127.1.1.21:3322")));
    assert_eq!(peers.next().unwrap().identifier(), "p1");
    assert_eq!(peers.next().unwrap().identifier(), "p2");
    assert_eq!(peers.next().unwrap().identifier(), "p3");
    assert_eq!(peers.next().unwrap().identifier(), "p1");
  }

}
