use super::*;
use std::net::ToSocketAddrs;

fn addr() -> SocketAddr {
  "127.1.1.11:3322".to_socket_addrs()
    .unwrap().into_iter().nth(0).unwrap()
}

fn has_change(diff: &Vec<Diff>, key: &str, value: Value, sequence: u64) -> bool {
  return diff.iter().any(|(k, (v, s))| {
      k == key && *v == value && *s == sequence
  });
}

#[test]
fn test_self_node_is_node() {
  let node = SelfNode::new("root".to_string(), addr());
  assert_eq!(node.identifier(), "root");
  assert_eq!(node.address().to_string(), "127.1.1.11:3322");
  assert_eq!(node.sequence(), 0);

  assert_eq!(node.digest(), ("root".into(), 0));
  assert!(node.get("buckets").is_none());
  assert!(node.diff(0).is_empty());
}

#[test]
fn test_self_node_set() {
  let mut node = SelfNode::new("root".into(), addr());
  node.set("buckets", vec![1, 5, 6].into());
  assert_eq!(node.sequence(), 1);
  assert_eq!(node.digest(), ("root".into(), 1));

  let v = node.get("buckets");
  assert!(v.is_some());
  assert_eq!(v.unwrap().as_integers().unwrap().as_slice(), [1, 5, 6]);

  let d = node.diff(0);
  assert_eq!(d.len(), 1);
  assert_eq!(d[0].0, "buckets".to_string());
  assert_eq!(d[0].1.0.as_integers().unwrap().as_slice(), [1, 5, 6]);
  assert_eq!(d[0].1.1, 1);

  assert!(node.diff(1).is_empty());
}

#[test]
fn test_self_node_multiple_sets() {
  let mut node = SelfNode::new("root".into(), addr());

  node.set("key1", 10.into());
  node.set("key2", "value".into());
  node.set("key1", 20.into()); // Overwrite key1

  assert_eq!(node.sequence(), 3);
  assert_eq!(node.get("key1"), Some(&20.into()));
  assert_eq!(node.get("key2"), Some(&"value".into()));

  let diff = node.diff(0);
  assert_eq!(diff.len(), 2);

  assert!(has_change(&diff, "key1", 20.into(), 3));
  assert!(has_change(&diff, "key2", "value".into(), 2));
}

#[test]
fn test_self_node_partial_diff() {
  let mut node = SelfNode::new("root".into(), addr());
  node.set("key1", 10.into());
  node.set("key2", "value".into());
  node.set("key3", true.into());

  let diff = node.diff(1);
  assert_eq!(diff.len(), 2);
  assert!(has_change(&diff, "key2", "value".into(), 2));
  assert!(has_change(&diff, "key3", true.into(), 3));
}

#[test]
fn test_self_node_is_not_discardable() {
  let mut node = SelfNode::new("root".into(), addr());
  assert_eq!(node.discardable(), false);
}

#[test]
fn test_peer_node_is_node() {
  let node = PeerNode::new("peer1".into(), addr());
  assert_eq!(node.identifier(), "peer1");
  assert_eq!(node.address().to_string(), "127.1.1.11:3322");
  assert_eq!(node.sequence(), 0);

  assert_eq!(node.digest(), ("peer1".into(), 0));
  assert!(node.get("buckets").is_none());
  assert!(node.diff(0).is_empty());
}

#[test]
fn test_peer_node_apply() {
  let mut node = PeerNode::new("peer1".to_string(), addr());
  node.apply(2, vec![
    ("key1".into(), (10.into(), 1)),
    ("key2".into(), ("value".into(), 2)),
  ]);

  assert_eq!(node.sequence(), 2);
  assert_eq!(node.get("key1"), Some(&10.into()));
  assert_eq!(node.get("key2"), Some(&"value".into()));
}

#[test]
fn test_peer_node_apply_outdated() {
  let mut node = PeerNode::new("peer1".to_string(), addr());
  node.apply(5, vec![("key1".into(), (10.into(), 5))]);
  node.apply(3, vec![("key2".into(), (20.into(), 3))]);
  node.apply(6, vec![("key1".into(), (99.into(), 5))]);

  assert_eq!(node.sequence(), 6);
  assert_eq!(node.get("key1"), Some(&10.into()));
  assert!(node.get("key2").is_none());
}

#[test]
fn test_peer_node_diff() {
  let mut node = PeerNode::new("peer1".to_string(), addr());
  node.apply(3, vec![
    ("key1".into(), (10.into(), 1)),
    ("key2".into(), (20.into(), 2)),
    ("key3".into(), (30.into(), 3)),
  ]);

  let diff = node.diff(1);
  assert_eq!(diff.len(), 2);
  assert!(has_change(&diff, "key2", 20.into(), 2));
  assert!(has_change(&diff, "key3", 30.into(), 3));
}

#[test]
fn test_peer_node_active() {
  let mut node = PeerNode::new("peer1".to_string(), addr());

  // Starts as inactive
  assert_eq!(node.active(), false);

  // Testing basic active/detector flow via private functions...

  // Becomes active when failure detector receives update
  node.update_detector();
  node.update_detector();
  assert_eq!(node.active(), true);

  // Becomes inactive when marked as inactive
  node.mark_inactive();
  assert_eq!(node.active(), false);
}

#[test]
fn test_peer_node_discardable() {
  let mut node = PeerNode::new("peer1".to_string(), addr());

  // With recent activity...
  node.update_detector();
  assert_eq!(node.active(), true);
  // not discardable and still active
  assert_eq!(node.discardable(), false);
  assert_eq!(node.active(), true);

  // Time passes...
  node.2.adjust(1e5);

  // With failing detector...
  let detector = node.1.as_ref().unwrap();
  assert_eq!(detector.failed(), false);
  Touch::set_global_now(1e2);
  assert_eq!(detector.failed(), true);
  // not discardable but now not active
  assert_eq!(node.discardable(), false);
  assert_eq!(node.active(), false);
  Touch::set_global_now(0.0);

  // Time passes...
  node.2.adjust(1e6);

  // Node becomes discardable
  assert_eq!(node.discardable(), true);

  // With new activity...
  node.update_detector();
  // no longer discardable and once again active
  assert_eq!(node.discardable(), false);
  assert_eq!(node.active(), true);
}
