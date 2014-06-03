use super::Database;
use super::options::{ReadOptions,WriteOptions};
use super::error::Error;

pub trait Interface {
  fn put(&mut self,
         options: WriteOptions,
         key: &[u8],
         value: &[u8])
        -> Result<(), Error>;
  fn delete(&mut self,
           options: WriteOptions,
           key: &[u8]) -> Result<(), Error>;
  fn get(&mut self,
         options: ReadOptions,
         key: &[u8]) -> Result<Option<Vec<u8>>, Error>;
}

impl Interface for Database {
  fn put(&mut self,
        options: WriteOptions,
        key: &[u8],
        value: &[u8]) -> Result<(), Error> {
    self.put_binary(options, key, value)
  }
  fn delete(&mut self,
            options: WriteOptions,
            key: &[u8]) -> Result<(), Error> {
    self.delete_binary(options, key)
  }
  fn get(&mut self,
         options: ReadOptions,
         key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
    self.get_binary(options, key)
  }
}
