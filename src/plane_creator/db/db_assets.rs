// //use rocksdb::{DB, Options};
// //delete this



// // NB: db is automatically closed at end of lifetime
// use bevy::prelude::*;
// use bevy::utils::HashMap;
// use rocksdb::DBWithThreadMode;
// use rocksdb::Error;
// use rocksdb::MultiThreaded;
// use rocksdb::Options;
// use serde::Serialize;
// use serde_json::Error as SError;

// //Input value into the database with given key
// //Key and Value should be serialized
// // currently db_worlds holds only one world
// pub fn put(key: String, value: String) -> Result<(), Error> {
//     let mut opts = Options::default();
//     opts.create_if_missing(true);
//     opts.increase_parallelism(3);
//     let path = "assets/database_assets";
//     pub type DB = DBWithThreadMode<MultiThreaded>;
//     let db = DB::open(&opts, path).expect("DB Open failed");
//     db.put(key, value)
// }

// //delete value from the database with given key
// pub fn delete(key: String) {
//     let mut opts = Options::default();
//     opts.create_if_missing(true);
//     opts.increase_parallelism(3);
//     pub type DB = DBWithThreadMode<MultiThreaded>;
//     let path = "assets/database_assets";
//     let db = DB::open(&opts, path).expect("DB Open failed");
//     db.delete(key).expect("delete failed");
// }

// //get value from the database with given key
// pub fn get(key: String) -> Result<Option<Vec<u8>>, Error> {
//     let mut opts = Options::default();
//     opts.create_if_missing(true);
//     opts.increase_parallelism(3);
//     pub type DB = DBWithThreadMode<MultiThreaded>;
//     let path = "assets/database_assets";
//     let db = DB::open(&opts, path).expect("DB Open failed");
//     db.get(key)
// }

// pub fn serialize<T: Serialize>(value: &T) -> Result<String, SError> {
//     serde_json::to_string(value)
// }

// pub fn deserialize<'a, T>(value: &'a Vec<u8>) -> Result<T, SError>
// where
//     T: serde::de::Deserialize<'a>,
// {
//     serde_json::from_str(std::str::from_utf8(value).expect("utf8 error"))
// }