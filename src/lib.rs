// use std::clone::Clone;

pub type Difficulty = u64;

type Hash = [u8; 32];
type KeyImage = Hash;

#[derive(Clone)]
pub struct Version {
  pub major: u8,
  pub minor: u8,
  pub patch: u8,
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let a: Hash = [0; 32];
    let b: KeyImage = [0; 32];
    assert_eq!(a, b);
  }
}
