use std::collections::HashSet;
use std::net::SocketAddr;

use indexmap::IndexMap;
use crate::node::{Node, PeerNode, Digest};
use crate::utils::{Rng, shuffle};

pub struct Peers {
  list: IndexMap<String, PeerNode>,
  offset: usize,
}

impl Peers {
  pub fn new() -> Self {
    Self {
      list: IndexMap::new(),
      offset: 0,
    }
  }

  pub fn len(&self) -> usize { self.list.len() }

  pub fn get(&self, identifier: &str) -> Option<&PeerNode> {
    self.list.get(identifier)
  }

  pub fn add(&mut self, node: PeerNode) {
    self.list.insert(node.identifier().to_owned(), node);
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

  fn next(&mut self) -> Option<&PeerNode> {
    self.offset += 1;
    match self.list.get_index(self.offset % self.list.len()) {
      Some((_, node)) => { Some(node) }
      None => { None }
    }
  }

  pub fn targets(&mut self) -> HashSet<SocketAddr> {
    let mut sample = HashSet::<SocketAddr>::new();

    // if self.len() == 0 {
    //   return roots
    // }
    let mut rng = Rng();

    // cycle through all peer nodes
    self.next().and_then(|n| Some(sample.insert(*n.address())));

    // sometimes, add a root
    if rng.rand_float() < 0.2 {
      // const address = this.randomRoot()
      // if (address) sample.add(address)
    }

    let (mut actives, inactives) = self.partition();

    // sometimes, add an inactive
    if rng.rand_float() < 0.1 {
      // const node = this.peers.randomInactive()
      // if (node) sample.add(node.address)
      let node = inactives[rng.rand_range(0 .. (inactives.len() as u64)) as usize];
      sample.insert(*node.address());
    }

    // add random actives to fill
    let count = usize::min(
      actives.len(),
      isize::max(0, (4 - sample.len() as isize)) as usize
    );
    shuffle(&mut rng, &mut actives, count);
    for n in &actives[0..count] {
      sample.insert(*n.address());
    }

    return sample;
  }
}
