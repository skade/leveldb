use super::Interface;
use super::Database;
use super::options::{ReadOptions,WriteOptions};
use super::error::Error;

pub struct Binary;

impl Interface<Binary, Vec<u8>> for Database {
  fn put(&mut self,
        options: WriteOptions,
        key: &[u8],
        value: Vec<u8>) -> Result<(), Error> {
    self.put_binary(options, key, value.as_slice())
  }
  fn delete(&mut self,
            options: WriteOptions,
            key: &[u8]) -> Result<(), Error> {
    self.delete_binary(options, key)
  }
  fn get(&self,
         options: ReadOptions,
         key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
    self.get_binary(options, key)
  }
}
