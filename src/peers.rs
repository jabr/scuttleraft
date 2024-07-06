use std::net::SocketAddr;
use std::iter::{Iterator, IntoIterator};

use fxhash::FxHashSet;

use indexmap::IndexMap;
use crate::node::{Node, PeerNode, Digest};
use crate::utils::{Rng, rng, rand};

pub struct Peers {
  list: IndexMap<String, PeerNode>,
  offset: usize,
  roots: Vec<SocketAddr>,
}

impl<'t> Peers {
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
