#[cfg(test)]
mod size {
     use utils::{open_database,tmpdir,db_put_simple};
     use leveldb::size::Size;

    #[test]
    fn test_iterator_from_to() {
        let tmp = tmpdir("size");
        let database = &mut open_database(tmp.path(), true);

        assert_eq!(database.approximate_size(&0, &1_000_000), 0);

        // We need to write a lot of data so it gets written to disk and size approximation works.
        for i in 0..1_000_000 {
            db_put_simple(database, i, &[0]);
        }

        assert!(database.approximate_size(&0, &1_000_000) > 1_000_000);
        assert!(database.approximate_size(&1, &1000) < 1_000_000);
        assert!(database.approximate_size(&1, &1000) > 1_000);
    }
}
