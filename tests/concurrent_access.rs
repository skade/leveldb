use leveldb::database::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use utils::{open_database, tmpdir};

#[test]
fn access_from_threads() {
    use std::sync::Arc;
    use std::thread;
    use std::thread::JoinHandle;

    let mut opts = Options::new();
    opts.create_if_missing = true;
    let tmp = tmpdir("sharing");
    let database = open_database(tmp.path(), true);
    let shared = Arc::new(database);

    (0..10)
        .map(|i| {
            let local_db = shared.clone();

            thread::spawn(move || {
                let write_opts = WriteOptions::new();
                match local_db.put(write_opts, i, &[i as u8]) {
                    Ok(_) => {}
                    Err(e) => {
                        panic!("failed to write to database: {:?}", e)
                    }
                }
            })
        })
        .map(JoinHandle::join)
        .for_each(drop);

    for i in 0..10 {
        assert_eq!(
            shared.get(ReadOptions::new(), i).unwrap(),
            Some(vec![i as u8])
        );
    }
}
