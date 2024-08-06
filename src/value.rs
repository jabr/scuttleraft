use std::convert::From;

#[derive(Clone)]
#[derive(Debug)]
pub enum Value {
  String(String),
  Boolean(bool),
  Integer(i64),
  Float(f64),
  Integers(Vec<i64>),
  Floats(Vec<f64>),
}

impl From<String> for Value {
  fn from(value: String) -> Self { Self::String(value) }
}

impl From<&str> for Value {
  fn from(value: &str) -> Self { Self::String(value.to_string()) }
}

impl From<bool> for Value {
  fn from(value: bool) -> Self { Self::Boolean(value) }
}

impl From<i64> for Value {
  fn from(value: i64) -> Self { Self::Integer(value) }
}

impl From<f64> for Value {
  fn from(value: f64) -> Self { Self::Float(value) }
}

impl From<Vec<i64>> for Value {
  fn from(value: Vec<i64>) -> Self { Self::Integers(value) }
}

impl From<&Vec<i64>> for Value {
  fn from(value: &Vec<i64>) -> Self { Self::Integers(value.clone()) }
}

impl From<Vec<i32>> for Value {
  fn from(value: Vec<i32>) -> Self {
    Self::Integers(value.into_iter().map(|v| v.into()).collect())
  }
}

impl From<&Vec<i32>> for Value {
  fn from(value: &Vec<i32>) -> Self {
    Self::Integers(value.into_iter().map(|&v| v.into()).collect())
  }
}

impl From<Vec<u32>> for Value {
  fn from(value: Vec<u32>) -> Self {
    Self::Integers(value.into_iter().map(|v| v.into()).collect())
  }
}

impl From<&Vec<u32>> for Value {
  fn from(value: &Vec<u32>) -> Self {
    Self::Integers(value.into_iter().map(|&v| v.into()).collect())
  }
}

impl From<Vec<i16>> for Value {
  fn from(value: Vec<i16>) -> Self {
    Self::Integers(value.into_iter().map(|v| v.into()).collect())
  }
}

impl From<&Vec<i16>> for Value {
  fn from(value: &Vec<i16>) -> Self {
    Self::Integers(value.into_iter().map(|&v| v.into()).collect())
  }
}

impl From<Vec<u16>> for Value {
  fn from(value: Vec<u16>) -> Self {
    Self::Integers(value.into_iter().map(|v| v.into()).collect())
  }
}

impl From<&Vec<u16>> for Value {
  fn from(value: &Vec<u16>) -> Self {
    Self::Integers(value.into_iter().map(|&v| v.into()).collect())
  }
}

impl From<Vec<i8>> for Value {
  fn from(value: Vec<i8>) -> Self {
    Self::Integers(value.into_iter().map(|v| v.into()).collect())
  }
}

impl From<&Vec<i8>> for Value {
  fn from(value: &Vec<i8>) -> Self {
    Self::Integers(value.into_iter().map(|&v| v.into()).collect())
  }
}

impl From<Vec<u8>> for Value {
  fn from(value: Vec<u8>) -> Self {
    Self::Integers(value.into_iter().map(|v| v.into()).collect())
  }
}

impl From<&Vec<u8>> for Value {
  fn from(value: &Vec<u8>) -> Self {
    Self::Integers(value.into_iter().map(|&v| v.into()).collect())
  }
}

impl From<Vec<f64>> for Value {
  fn from(value: Vec<f64>) -> Self { Self::Floats(value) }
}

impl From<&Vec<f64>> for Value {
  fn from(value: &Vec<f64>) -> Self { Self::Floats(value.clone()) }
}

impl From<Vec<f32>> for Value {
  fn from(value: Vec<f32>) -> Self {
    Self::Floats(value.into_iter().map(|v| v.into()).collect())
  }
}

impl From<&Vec<f32>> for Value {
  fn from(value: &Vec<f32>) -> Self {
    Self::Floats(value.into_iter().map(|&v| v.into()).collect())
  }
}

impl Value {
  pub fn as_string(&self) -> Option<&str> {
    match self {
      Self::String(v) => { Some(v) }
      _ => { None }
    }
  }

  pub fn as_bool(&self) -> Option<bool> {
    match self {
      Self::Boolean(v) => { Some(*v) }
      _ => { None }
    }
  }

  pub fn as_integer(&self) -> Option<i64> {
    match self {
      Self::Integer(v) => { Some(*v) }
      _ => { None }
    }
  }

  pub fn as_float(&self) -> Option<f64> {
    match self {
      Self::Float(v) => { Some(*v) }
      _ => { None }
    }
  }

  pub fn as_integers(&self) -> Option<&Vec<i64>> {
    match self {
      Self::Integers(v) => { Some(v) }
      _ => { None }
    }
  }

  pub fn as_floats(&self) -> Option<&Vec<f64>> {
    match self {
      Self::Floats(v) => { Some(v) }
      _ => { None }
    }
  }
}
