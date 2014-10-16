use super::Interface;
use super::Database;
use super::options::{ReadOptions,WriteOptions};
use super::error::Error;
use serialize::{Encodable, Decodable, Encoder, Decoder};
use serialize::json;
use serialize::json::{DecodeResult, DecoderError};
use std::io::IoError;
use std::str::from_utf8;

//pub trait Interface<'a, T:Encodable<json::Encoder<'a>,io::IoError>> {
//  fn put(&mut self,
//         options: WriteOptions,
//         key: &T,
//         value: &T)
//        -> Result<(), Error>;
//  fn delete(&mut self,
//           options: WriteOptions,
//           key: &T) -> Result<(), Error>;
//  fn get(&self,
//         options: ReadOptions,
//         key: &T) -> Result<Option<Json>, Error>;
//}

pub struct JSON;

impl<'a, V: Encodable<json::Encoder<'a>, IoError>, R: Decodable<json::Decoder, json::DecoderError>> Interface<JSON, V, R> for Database {
  fn put(&mut self,
        options: WriteOptions,
        key: &[u8],
        value: V) -> Result<(), Error> {
    let encoded_val = json::Encoder::buffer_encode(&value);
    self.put_binary(options, key, encoded_val.as_slice())
  }
  fn delete(&mut self,
            options: WriteOptions,
            key: &[u8]) -> Result<(), Error> {
    let encoded_key = json::Encoder::buffer_encode(&key);
    self.delete_binary(options, encoded_key.as_slice())
  }
  fn get(&self,
         options: ReadOptions,
         key: &[u8]) -> Result<Option<R>, Error> {
    let result = self.get_binary(options, key);
    match result {
      Err(error) => { Err(error) },
      Ok(opt) => {
        match opt {
          None => { Ok(None) },
          Some(binary) => {
            let reader = from_utf8(binary.as_slice())
                             .unwrap();

            let decoded: DecodeResult<R> = json::decode(reader);
            match decoded {
              Ok(o) => { Ok(Some(o)) },
              Err(_) => { Err( Error::new(from_str("json parsing failed").unwrap()) ) }
            }
          }
        }
      }
    }
  }
}
