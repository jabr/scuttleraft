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
