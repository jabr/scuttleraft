use super::*;
use std::net::ToSocketAddrs;

fn addr() -> SocketAddr {
  "127.1.1.11:3322".to_socket_addrs()
    .unwrap().into_iter().nth(0).unwrap()
}

#[test]
fn test_self_node_is_node() {
  let node = SelfNode::new("root".to_string(), addr());
  assert_eq!(node.identifier(), "root");
  assert_eq!(node.address().to_string(), "127.1.1.11:3322");
  assert_eq!(node.sequence(), 0);
  assert_eq!(node.digest(), ("root".to_string(), 0));
  assert!(node.get("buckets").is_none());
  assert!(node.diff(0).is_empty());
}

#[test]
fn test_self_node_set() {
  let mut node = SelfNode::new("root".to_string(), addr());
  assert_eq!(node.sequence(), 0);
  assert_eq!(node.digest(), ("root".to_string(), 0));
  assert!(node.get("buckets").is_none());
  assert!(node.diff(0).is_empty());
  let buckets: Value = vec![1, 5, 6].into();
  node.set("buckets", buckets);
  assert_eq!(node.sequence(), 1);
  assert_eq!(node.digest(), ("root".to_string(), 1));
  let v = node.get("buckets");
  assert!(v.is_some());
  assert_eq!(v.unwrap().as_integers().unwrap().as_slice(), [1, 5, 6]);
  let d = node.diff(0);
  assert_eq!(d.len(), 1);
  assert_eq!(d[0].0, "buckets".to_string());
  assert_eq!(d[0].1 .0.as_integers().unwrap().as_slice(), [1, 5, 6]);
  assert_eq!(d[0].1 .1, 1);
  assert!(node.diff(1).is_empty());
}

#[test]
fn test_self_node_multiple_sets() {
  let mut node = SelfNode::new("root".to_string(), addr());
  node.set("key1", Value::Integer(10));
  node.set("key2", Value::String("value".to_string()));
  node.set("key1", Value::Integer(20)); // Overwrite key1

  assert_eq!(node.sequence(), 3);
  assert_eq!(node.get("key1").unwrap().as_integer().unwrap(), 20);
  assert_eq!(node.get("key2").unwrap().as_string().unwrap(), "value");

  let diff = node.diff(0);
  assert_eq!(diff.len(), 2);
  assert!(diff
    .iter()
    .any(|(k, (v, s))| k == "key1" && v.as_integer().unwrap() == 20 && *s == 3));
  assert!(diff
    .iter()
    .any(|(k, (v, s))| k == "key2" && v.as_string().unwrap() == "value" && *s == 2));
}

#[test]
fn test_self_node_partial_diff() {
  let mut node = SelfNode::new("root".to_string(), addr());
  node.set("key1", Value::Integer(10));
  node.set("key2", Value::String("value".to_string()));
  node.set("key3", Value::Boolean(true));

  let diff = node.diff(1);
  assert_eq!(diff.len(), 2);
  assert!(diff
    .iter()
    .any(|(k, (v, s))| k == "key2" && v.as_string().unwrap() == "value" && *s == 2));
  assert!(diff
    .iter()
    .any(|(k, (v, s))| k == "key3" && v.as_bool().unwrap() == true && *s == 3));
}

#[test]
fn test_self_node_is_not_discardable() {
  let mut node = SelfNode::new("root".to_string(), addr());
  assert_eq!(node.discardable(), false);
}

#[test]
fn test_peer_node_is_node() {
  let node = PeerNode::new("peer1".to_string(), addr());
  assert_eq!(node.identifier(), "peer1");
  assert_eq!(node.address().to_string(), "127.1.1.11:3322");
  assert_eq!(node.sequence(), 0);
  assert_eq!(node.digest(), ("peer1".to_string(), 0));
  assert!(node.get("buckets").is_none());
  assert!(node.diff(0).is_empty());
}

#[test]
fn test_peer_node_apply() {
  let mut node = PeerNode::new("peer1".to_string(), addr());
  node.apply(
    2,
    vec![
    ("key1".to_string(), (10.into(), 1)),
    ("key2".to_string(), ("value".into(), 2)),
    ],
  );

  assert_eq!(node.sequence(), 2);
  assert_eq!(node.get("key1"), Some(&Value::Integer(10)));
  assert_eq!(node.get("key2"), Some(&"value".into()));
}

#[test]
fn test_peer_node_apply_outdated() {
  let mut node = PeerNode::new("peer1".to_string(), addr());
  node.apply(5, vec![("key1".to_string(), (Value::Integer(10), 5))]);
  node.apply(3, vec![("key2".to_string(), (Value::Integer(20), 3))]);

  assert_eq!(node.sequence(), 5);
  assert_eq!(node.get("key1"), Some(&10.into()));
  assert!(node.get("key2").is_none());
}

#[test]
fn test_peer_node_diff() {
  let mut node = PeerNode::new("peer1".to_string(), addr());
  node.apply(
    3,
    vec![
    ("key1".to_string(), (10.into(), 1)),
    ("key2".to_string(), (20.into(), 2)),
    ("key3".to_string(), (30.into(), 3)),
    ],
  );

  let diff = node.diff(1);
  assert_eq!(diff.len(), 2);
  let (key, (value, sequence)) = &diff[0];
  assert_eq!(key, "key2");
  assert_eq!(value.as_integer(), Some(20));
  assert_eq!(*sequence, 2);
  let (key, (value, sequence)) = &diff[1];
  assert_eq!(key, "key3");
  assert_eq!(value.as_integer(), Some(30));
  assert_eq!(*sequence, 3);
}

#[test]
fn test_peer_node_discardable() {
  let mut node = PeerNode::new("peer1".to_string(), addr());

  // Not discardable when newly created
  assert!(!node.discardable());

  // Simulate passage of time...
  node.2.adjust(1e6);
  // Becomes discardable eventually
  assert!(node.discardable());

  node.apply(1, vec![]); // This should make the node active...
  assert!(node.active());
  // and no longer discardable.
  assert!(!node.discardable());
}
