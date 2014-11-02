use super::Interface;
use super::Database;
use super::options::{ReadOptions,WriteOptions};
use super::error::Error;
use super::key::Key;
use comparator::Comparator;

pub struct Binary;

impl<K: Key, C: Comparator<K>> Interface<Binary, K, Vec<u8>> for Database<C> {
  fn put(&mut self,
        options: WriteOptions,
        key: K,
        value: Vec<u8>) -> Result<(), Error> {
    self.put_binary(options, key, value.as_slice())
  }
  fn delete(&mut self,
            options: WriteOptions,
            key: K) -> Result<(), Error> {
    self.delete_binary(options, key)
  }
  fn get(&self,
         options: ReadOptions,
         key: K) -> Result<Option<Vec<u8>>, Error> {
    self.get_binary(options, key)
  }
}
