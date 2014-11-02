use super::Interface;
use super::Database;
use super::error::Error;
use super::key::Key;
use comparator::Comparator;

pub struct Binary;

impl<K: Key, C: Comparator<K>> Interface<Binary, K, Vec<u8>> for Database<C> {
  fn from_binary(&self, binary: Vec<u8>) -> Result<Option<Vec<u8>>, Error> {
    Ok(Some(binary))
  }
  fn to_binary(&mut self, val: Vec<u8>) -> Vec<u8> {
    val
  }
}
