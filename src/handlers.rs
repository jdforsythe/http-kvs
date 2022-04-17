use std::convert::Infallible;

use warp::{ self, http::StatusCode };
use bytes::Bytes;

use crate::db::Db;

pub(crate) async fn set_value(key: String, value: Bytes, db: Db) -> Result<impl warp::Reply, Infallible> {
  db.set(key, value);

  Ok(warp::reply::with_status("OK", StatusCode::OK))
}

pub(crate) async fn get_value(key: String, db: Db) -> Result<Box<dyn warp::Reply>, Infallible> {
  let val = db.get(&key);

  match val {
    Some(x) => return Ok(Box::new(warp::reply::with_status(String::from_utf8(x.to_vec()).unwrap(), StatusCode::OK))),
    None => return Ok(Box::new(StatusCode::NOT_FOUND)),
  }
}
