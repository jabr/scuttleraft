use std::net::SocketAddr;
use fxhash::FxHashMap;

use crate::value::Value;
use crate::failure_detector::FailureDetector;

#[cfg(not(test))]
use crate::utils::Touch;

#[cfg(test)]
use crate::utils::testing::ManualTouch as Touch;

type SequencedValue = (Value, u64);
pub type Diff = (String, (Value, u64));
pub type Digest = (String, u64);

pub trait Node {
  fn identifier(&self) -> &str;
  fn address(&self) -> &SocketAddr;
  fn sequence(&self) -> u64;
  fn digest(&self) -> Digest;
  fn get(&self, key: &str) -> Option<&Value>;
  fn diff(&self, from: u64) -> Vec<Diff>;
  fn discardable(&mut self) -> bool;
}

struct BaseNode {
  identifier: String,
  address: SocketAddr,
  sequence: u64,
  values: FxHashMap<String, SequencedValue>,
}

impl BaseNode {
  fn new(identifier: String, address: SocketAddr) -> Self {
    Self {
      identifier: identifier,
      address,
      sequence: 0,
      values: FxHashMap::default(),
    }
  }

  fn identifier(&self) -> &str { self.identifier.as_str() }
  fn address(&self) -> &SocketAddr { &self.address }
  fn sequence(&self) -> u64 { self.sequence }

  fn digest(&self) -> Digest {
    (self.identifier.clone(), self.sequence)
  }

  fn get(&self, key: &str) -> Option<&Value> {
    self.values.get(key).and_then(|(v,_)| Some(v))
  }

  fn diff(&self, from: u64) -> Vec<Diff> {
    self.values.iter()
      .filter(|(_, &(_, s))| s > from)
      .map(|(k, (v, s))| (k.clone(), (v.clone(), *s)))
      .collect()
  }
}

pub struct SelfNode(BaseNode);

impl SelfNode {
  pub fn new(identifier: String, address: SocketAddr) -> Self {
    Self(BaseNode::new(identifier, address))
  }

  fn set(&mut self, key: &str, value: Value) {
    self.0.sequence += 1;
    self.0.values.insert(
      key.to_string(),
      (value, self.0.sequence)
    );
  }
}

impl Node for SelfNode {
  fn identifier(&self) -> &str { self.0.identifier() }
  fn address(&self) -> &SocketAddr { self.0.address() }
  fn sequence(&self) -> u64 { self.0.sequence() }
  fn digest(&self) -> Digest { self.0.digest() }
  fn get(&self, key: &str) -> Option<&Value> { self.0.get(key) }
  fn diff(&self, from: u64) -> Vec<Diff> { self.0.diff(from) }
  fn discardable(&mut self) -> bool { false }
}

pub struct PeerNode(BaseNode, Option<FailureDetector>, Touch);

impl PeerNode {
  pub fn new(identifier: String, address: SocketAddr) -> Self {
    Self(BaseNode::new(identifier, address), None, Touch::now())
  }

  pub fn active(&self) -> bool { self.1.is_some() }

  fn mark_inactive(&mut self) {
    self.1 = None;
    self.2 = Touch::now();
  }

  fn update_detector(&mut self) {
    match &mut self.1 {
      // if detector exists, update the detector
      Some(d) => { d.update(); }
      // otherwise, create a new detector
      None => {
        self.1 = Some(FailureDetector::default());
      }
    }
  }

  fn current_sequence_for(&self, key: &str) -> u64 {
    let default = (Value::Boolean(false), 0); // default to sequence 0
    return self.0.values.get(key).unwrap_or(&default).1;
  }

  pub fn apply(&mut self, sequence: u64, updates: Vec<Diff>) {
    // is update older than our current data?
    if sequence < self.0.sequence { return; }

    self.update_detector();

    for (k, (v, s)) in updates {
      if s > self.current_sequence_for(k.as_str()) {
        // update value when sequence is newer
        self.0.values.insert(k, (v, s));
      }
    }

    self.0.sequence = sequence;
  }
}

impl Node for PeerNode {
  fn identifier(&self) -> &str { self.0.identifier() }
  fn address(&self) -> &SocketAddr { self.0.address() }
  fn sequence(&self) -> u64 { self.0.sequence }
  fn digest(&self) -> Digest { self.0.digest() }
  fn get(&self, key: &str) -> Option<&Value> { self.0.get(key) }
  fn diff(&self, from: u64) -> Vec<Diff> { self.0.diff(from) }

  fn discardable(&mut self) -> bool {
    match &self.1 {
      Some(d) => {
        if d.failed() { self.mark_inactive(); }
        return false;
      }
      None => {
        return self.2.age() > 86_400.0;
      }
    }
  }
}

#[cfg(test)]
mod test;
