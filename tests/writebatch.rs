use leveldb::database::batch::{Batch, Writebatch, WritebatchIterator};
use leveldb::database::kv::KV;
use leveldb::database::Database;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use utils::tmpdir;

#[test]
fn test_writebatch() {
    let mut opts = Options::new();
    opts.create_if_missing = true;
    let tmp = tmpdir("writebatch");
    let database = &mut Database::open(tmp.path(), opts).unwrap();
    let batch = &mut Writebatch::new();
    batch.put(1, &[1]);
    batch.put(2, &[2]);
    batch.delete(1);
    let wopts = WriteOptions::new();
    let ack = database.write(wopts, batch);
    assert!(ack.is_ok());

    let read_opts = ReadOptions::new();
    let res = database.get(read_opts, 2);

    match res {
        Ok(data) => {
            assert!(data.is_some());
            let data = data.unwrap();
            assert_eq!(data, vec!(2));
        }
        Err(_) => panic!("failed reading data"),
    }

    let read_opts2 = ReadOptions::new();
    let res2 = database.get(read_opts2, 1);
    match res2 {
        Ok(data) => assert!(data.is_none()),
        Err(_) => panic!("failed reading data"),
    }
}

struct Iter {
    put: i32,
    deleted: i32,
}

impl WritebatchIterator for Iter {
    type K = i32;

    fn put(&mut self, _key: i32, _value: &[u8]) {
        self.put = self.put + 1;
    }

    fn deleted(&mut self, _key: i32) {
        self.deleted = self.deleted + 1;
    }
}

#[test]
fn test_writebatchiter() {
    let mut opts = Options::new();
    opts.create_if_missing = true;
    let tmp = tmpdir("writebatch");
    let database = &mut Database::open(tmp.path(), opts).unwrap();
    let batch = &mut Writebatch::new();
    batch.put(1, &[1]);
    batch.put(2, &[2]);
    batch.delete(1);

    let wopts = WriteOptions::new();
    let ack = database.write(wopts, batch);
    assert!(ack.is_ok());

    let iter = Box::new(Iter { put: 0, deleted: 0 });
    let iter2 = batch.iterate(iter);
    assert_eq!(iter2.put, 2);
    assert_eq!(iter2.deleted, 1);
}
