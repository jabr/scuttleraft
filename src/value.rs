use std::convert::From;

#[derive(Clone, Debug, PartialEq)]
pub enum Number {
  Unsigned(u64),
  Signed(i64),
  Float(f64),
}

impl From<u64> for Number {
  fn from(value: u64) -> Self { Self::Unsigned(value) }
}

impl From<u32> for Number {
  fn from(value: u32) -> Self { Self::Unsigned(value as u64) }
}

impl From<usize> for Number {
  fn from(value: usize) -> Self { Self::Unsigned(value as u64) }
}

impl From<u16> for Number {
  fn from(value: u16) -> Self { Self::Unsigned(value as u64) }
}

impl From<u8> for Number {
  fn from(value: u8) -> Self { Self::Unsigned(value as u64) }
}

impl From<i64> for Number {
  fn from(value: i64) -> Self { Self::Signed(value) }
}

impl From<i32> for Number {
  fn from(value: i32) -> Self { Self::Signed(value as i64) }
}

impl From<isize> for Number {
  fn from(value: isize) -> Self { Self::Signed(value as i64) }
}

impl From<i16> for Number {
  fn from(value: i16) -> Self { Self::Signed(value as i64) }
}

impl From<i8> for Number {
  fn from(value: i8) -> Self { Self::Signed(value as i64) }
}

impl From<f64> for Number {
  fn from(value: f64) -> Self { Self::Float(value) }
}

impl From<f32> for Number {
  fn from(value: f32) -> Self { Self::Float(value as f64) }
}

impl Number {
  fn is_unsigned(&self) -> bool {
    match self {
      Self::Unsigned(_) => true,
      _ => false
    }
  }

  fn as_unsigned(&self) -> Option<u64> {
    match self {
      Self::Unsigned(n) => Some(*n),
      _ => None
    }
  }

  fn is_signed(&self) -> bool {
    match self {
      Self::Signed(_) => true,
      _ => false
    }
  }

  fn as_signed(&self) -> Option<i64> {
    match self {
      Self::Signed(n) => Some(*n),
      _ => None
    }
  }

  fn is_float(&self) -> bool {
    match self {
      Self::Float(_) => true,
      _ => false
    }
  }

  fn as_float(&self) -> Option<f64> {
    match self {
      Self::Float(n) => Some(*n),
      _ => None
    }
  }
}

#[derive(PartialEq, Clone, Debug)]
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_number_unsigned() {
    let u = Number::Unsigned(44);
    assert!(u.is_unsigned());
    assert_eq!(Some(44), u.as_unsigned());
    assert!(!u.is_signed());
    assert_eq!(None, u.as_signed());
    assert!(!u.is_float());
    assert_eq!(None, u.as_float());
  }

  #[test]
  fn test_number_signed() {
    let u = Number::Signed(44);
    assert!(!u.is_unsigned());
    assert_eq!(None, u.as_unsigned());
    assert!(u.is_signed());
    assert_eq!(Some(44), u.as_signed());
    assert!(!u.is_float());
    assert_eq!(None, u.as_float());
  }

  #[test]
  fn test_number_float() {
    let u = Number::Float(44.0);
    assert!(!u.is_unsigned());
    assert_eq!(None, u.as_unsigned());
    assert!(!u.is_signed());
    assert_eq!(None, u.as_signed());
    assert!(u.is_float());
    assert_eq!(Some(44.0), u.as_float());
  }

  #[test]
  fn test_number_into() {
    assert_eq!(Number::Unsigned(44), 44u64.into());
    assert_eq!(Number::Unsigned(44), 44u32.into());
    assert_eq!(Number::Unsigned(44), 44usize.into());
    assert_eq!(Number::Unsigned(44), 44u16.into());
    assert_eq!(Number::Unsigned(44), 44u8.into());

    assert_eq!(Number::Signed(44), 44i64.into());
    assert_eq!(Number::Signed(44), 44i32.into());
    assert_eq!(Number::Signed(44), 44isize.into());
    assert_eq!(Number::Signed(44), 44i16.into());
    assert_eq!(Number::Signed(44), 44i8.into());

    assert_eq!(Number::Float(44.0), 44f64.into());
    assert_eq!(Number::Float(44.0), 44f32.into());

    assert_eq!(Number::Signed(44), 44.into());
  }
}
