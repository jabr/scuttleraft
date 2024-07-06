use std::net::SocketAddr;

use crate::node::{Node, SelfNode, Diff, Digest};
use crate::peers::Peers;

struct Gossip {
  name: String,
  node: SelfNode,
  peers: Peers,
}

type NodeDiff<'t> = (Digest<'t>, Vec<Diff<'t>>, Option<SocketAddr>);

impl Gossip {

  fn process_requests<'a>(&self, requests: Vec<Digest<'a>>) -> Vec<NodeDiff<'a>> {
    let mut diffs: Vec<NodeDiff<'a>> = Vec::new();

    for (identifier, sequence) in requests {
      let mut node: Option<&dyn Node> = None;
      if identifier == self.node.identifier() { node = Some(&self.node); }
      if node.is_none() {
        node = self.peers.get(identifier).and_then(|n| Some(n as &dyn Node));
      }
      match node {
        Some(n) => {
          if n.sequence() > sequence {
            let mut diff: NodeDiff = (n.digest(), n.diff(sequence), None);
            if sequence == 0 { diff.2 = Some(*n.address()); }
            diffs.push(diff);
          }
        }
        None => {}
      }
    }

    return diffs;
  }
}
