#[cfg(test)]
mod compaction {
    use leveldb::compaction::Compaction;
    use utils::{db_put_simple, open_database, tmpdir};

    #[test]
    fn test_iterator_from_to() {
        let tmp = tmpdir("compact");
        let database = &mut open_database(tmp.path(), true);
        db_put_simple(database, 1, &[1]);
        db_put_simple(database, 2, &[2]);
        db_put_simple(database, 3, &[3]);
        db_put_simple(database, 4, &[4]);
        db_put_simple(database, 5, &[5]);

        let from = 2;
        let to = 4;
        database.compact(&from, &to);
    }
}
