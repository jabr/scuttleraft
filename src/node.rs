use std::collections::HashMap; // @todo: better option?
use std::net::SocketAddr;

use crate::Value;

use crate::failure_detector::FailureDetector;
use crate::utils::Touch;

type SequencedValue = (Value, u64);
type Diff = (&String, (&Value, u64));
type Digest = (String, u64);

pub trait Node {
  fn digest(&self) -> Digest;
  fn get(&self, key: &str) -> Option<&Value>;
  fn diff(&self, rom: u64) -> [Diff];
  fn discardable(&self) -> bool;
}

struct BaseNode {
  identifier: String,
  address: Option<SocketAddr>,
  sequence: u64,
  values: HashMap<String, SequencedValue>,
}

impl BaseNode {
  fn new(identifier: &str) -> Self {
    Self {
      identifier: identifier.to_string(),
      address: None,
      sequence: 0,
      values: HashMap::new(),
    }
  }

  fn digest(&self) -> Digest { (self.identifier.clone(), self.sequence) }

  fn get(&self, key: &str) -> Option<&Value> {
    self.values.get(key).and_then(|(v,_)| Some(v))
  }

  fn diff(&self, from: u64) -> [Diff] {
    self.values.iter()
      .filter(|&(k, &(v, s))| s > from)
      .map(|(k, &(v, s))| (k.clone(), (v, s)))
      .collect()
  }
}

pub struct SelfNode(BaseNode);

impl SelfNode {
  fn set(&mut self, key: &str, value: Value) {
    self.0.sequence += 1;
    self.0.values.insert(
      key.to_string(),
      (value, self.0.sequence)
    );
  }
}

impl Node for SelfNode {
  fn digest(&self) -> Digest { self.0.digest() }
  fn get(&self, key: &str) -> Option<&Value> { self.0.get(key) }
  fn diff(&self, from: u64) -> [Diff] { self.0.diff(from) }
  fn discardable(&self) -> bool { false }
}

pub struct PeerNode(BaseNode, Option<FailureDetector>, Touch);

impl PeerNode {
  fn active(&self) -> bool { self.1.is_some() }

  fn mark_inactive(&mut self) {
    self.1 = None;
    self.2 = Touch::now();
  }

  fn apply() {} // @todo
}

impl Node for PeerNode {
  fn digest(&self) -> Digest { self.0.digest() }
  fn get(&self, key: &str) -> Option<&Value> { self.0.get(key) }
  fn diff(&self, from: u64) -> [Diff] { self.0.diff(from) }

  fn discardable(&self) -> bool {
    match self.1 {
      Some(d) => {
        if d.failed() {
          self.mark_inactive();
        }
        return false;
      },
      None => {
        return self.2.elapsed() > 100.0;
      }
    }
  }
}
