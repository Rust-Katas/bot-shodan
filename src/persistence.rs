use lazy_static::lazy_static;
use rocksdb::{DB, Options};


/*
 * We will allow an admin user to give other users permissions to party
 */

lazy_static! {
    pub static ref XDB: DB = open_db();
}

/**
 * 创建一个新的数据库
 */
fn open_db() -> DB {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    DB::open(&opts, "./rocks_database").unwrap()
}

/**
 * for each user that has permission, we will insert a key-value pair
 * user_${username} and value is the permission
 */
fn give_user_permission() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn create_and_destroy_db() {
    let path = "_path_for_rocksdb_storage";
    {
        let db = DB::open_default(path).unwrap();
        //db.put(b"my key", b"my value").unwrap();
        match db.get(b"my key") {
            Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
        //db.delete(b"my key").unwrap();
    }
    //let _ = DB::destroy(&Options::default(), path);
    }
}
