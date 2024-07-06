use std::net::SocketAddr;

use crate::node::{Node, SelfNode, PeerNode, Diff, Digest};
use crate::peers::Peers;

struct Gossip {
  name: String,
  node: SelfNode,
  peers: Peers,
}

type NodeDiff = (Digest, Vec<Diff>, Option<SocketAddr>);

impl Gossip {

  fn process_digest(&self, digest: Vec<Digest>) -> (Vec<Digest>, Vec<NodeDiff>) {
    let mut requests: Vec<Digest> = Vec::new();
    let mut diffs: Vec<NodeDiff> = Vec::new();
    let mut actives = self.peers.actives();
    let mut seen_self = false;

    for (identifier, sequence) in digest {
      if self.node.identifier() == identifier {
        seen_self = true;
        let n = &self.node;
        let node_sequence = self.node.sequence();
        if n.sequence() < sequence {
            // we received a digest claiming to have a newer version of ourself. this should not happen.
            // console.error(`received digest for ourself with a higher sequence (${identifier} @ ${sequence} > ${node.sequence})`)
            // @note: should we shutdown? something else? jump sequence and send update with current state?
        } else if node_sequence > sequence {
          diffs.push((n.digest(), n.diff(sequence), None));
        }
        continue;
      }

      match self.peers.get(identifier.as_str()) {
        Some(n) => {
          actives.remove(n.identifier());
          let node_sequence = n.sequence();
          if node_sequence < sequence {
            requests.push((identifier, node_sequence));
          } else if node_sequence > sequence {
            diffs.push((n.digest(), n.diff(sequence), None));
          }
        }
        None => {
          // unknown node, so request all info on it.
          requests.push((identifier, 0));
        }
      }
    }

    // add diffs our ourself if we weren't in the digest
    if !seen_self {
      let n = &self.node;
      diffs.push((n.digest(), n.diff(0), Some(*n.address())));
    }

    // add diffs for any active nodes we have that are not in the digest
    for node in actives.into_values() {
      diffs.push((node.digest(), node.diff(0), Some(*node.address())));
    }

    return (requests, diffs);
  }

  fn process_diffs(&mut self, diffs: Vec<NodeDiff>) {
    for ((identifier, sequence), updates, address) in diffs {
      if self.node.identifier() == identifier {
        // we received an update for ourself. this should not happen.
        // console.error(`received diffs to update ourself (${identifier} @ ${sequence})`)
        // @note: should we shutdown? something else?
        continue;
      }

      match self.peers.get(identifier.as_str()) {
        Some(n) => { n.apply(sequence, updates); }
        None => {
          match address {
            Some(a) => {
              let mut new_node = PeerNode::new(identifier, a);
              new_node.apply(sequence, updates);
              self.peers.add(new_node);
            }
            None => {
              // @todo: log unknown node with no address
              return;
            }
          }
        }
      }
    }
  }

  fn process_requests(&self, requests: Vec<Digest>) -> Vec<NodeDiff> {
    let mut diffs: Vec<NodeDiff> = Vec::new();

    let mut add = |n: &dyn Node, sequence: u64| {
      if n.sequence() > sequence {
          let mut diff: NodeDiff = (n.digest(), n.diff(sequence), None);
        if sequence == 0 { diff.2 = Some(*n.address()); }
        diffs.push(diff);
      }
    };

    for (identifier, sequence) in requests {
      if self.node.identifier() == identifier { add(&self.node, sequence); }
      match self.peers.get(identifier.as_str()) {
        Some(n) => { add(n, sequence); }
        None => {
          // @todo: log unknown node
        }
      }
    }

    return diffs;
  }
}
