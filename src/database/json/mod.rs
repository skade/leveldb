use database;
use super::Database;
use super::error::Error;
use super::key::Key;
use comparator::Comparator;
use serialize::{Encodable, Decodable, Encoder, Decoder};
use serialize::json;
use serialize::json::{DecodeResult, DecoderError};
use std::io::IoError;
use std::str::from_utf8;

pub struct JSON;

impl<'a, K: Key, C: Comparator<K>, V: Encodable<json::Encoder<'a>, IoError> + Decodable<json::Decoder, json::DecoderError>> database::Interface<JSON, K, V> for Database<C> {
  fn from_binary(&self, binary: Vec<u8>) -> Result<Option<V>, Error> {
    let reader = from_utf8(binary.as_slice())
                     .unwrap();

    let decoded: DecodeResult<V> = json::decode(reader);
    match decoded {
      Ok(o) => { Ok(Some(o)) },
      Err(_) => { Err( Error::new(from_str("json parsing failed").unwrap()) ) }
    }
  }

  fn to_binary(&mut self, val: V) -> Vec<u8> {
    json::Encoder::buffer_encode(&val)
  }
}
