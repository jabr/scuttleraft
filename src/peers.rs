use indexmap::IndexMap;
use crate::node::{Node, PeerNode};

type Digest<'a> = (&'a str, u64);

pub struct Peers {
  list: IndexMap<String, PeerNode>,
  offset: usize,
}

impl Peers {
  fn new() -> Self {
    Self {
      list: IndexMap::new(),
      offset: 0,
    }
  }

  fn len(&self) -> usize { self.list.len() }

  fn get(&self, identifier: &str) -> Option<&PeerNode> {
    self.list.get(identifier)
  }

  fn add(&mut self, node: PeerNode) {
    self.list.insert(node.identifier(), node);
  }

  fn digest(&self) -> Vec<Digest> {
    self.list.iter().map(|(_,n)| n.digest()).collect()
  }

  fn prune(&mut self) {
    self.list.retain(|_,n| !n.discardable());
  }

  // randomActives(count: usize) -> Vec<PeerNode>
  // randomInactive(count) -> Option<PeerNode>

  fn next(&mut self) -> Option<&PeerNode> {
    self.offset += 1;
    match self.list.get_index(self.offset % self.list.len()) {
      Some((_, node)) => { Some(node) }
      None => { None }
    }
  }

}
