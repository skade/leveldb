use utils::{tmpdir};
use leveldb::database::{Database};
use leveldb::options::{Options,ReadOptions,WriteOptions};
use leveldb::database::kv::{KV};
use leveldb::database::cache::{Cache};
use leveldb::database::batch::{Batch,Writebatch};

#[test]
fn test_writebatch() {
    let mut opts = Options::new();
    opts.create_if_missing = true;
    let tmp = tmpdir("writebatch");
    let database = &mut Database::open(tmp.path(), opts).unwrap();
    let mut batch = Writebatch::new();
    batch.put(1, &[1]);
    batch.put(2, &[2]);
    batch.delete(1);
    let wopts = WriteOptions::new();
    database.write(wopts, batch);

    let read_opts = ReadOptions::new();
    let res = database.get(read_opts, 2);

    match res {
        Ok(data) => {
            assert!(data.is_some());
            let data = data.unwrap();
            assert_eq!(data, vec!(2));
        },
        Err(_) => { panic!("failed reading data") }
    }

    let read_opts2 = ReadOptions::new();
    let res2 = database.get(read_opts2, 1);
    match res2 {
        Ok(data) => { assert!(data.is_none()) },
        Err(_) => { panic!("failed reading data") }
    }
}
