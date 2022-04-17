use std::convert::Infallible;

use bytes::Bytes;
use warp::{ self, Filter };

use crate::db::Db;
use crate::handlers;

/// Allows the data store to be injected into the route and passed
/// along into the handler.
fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
  warp::any().map(move || db.clone())
}

// Injects the body, in bytes, into the route and passes into the handler
fn with_body() -> impl Filter<Extract = (Bytes,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(1024 * 16)
    .and(warp::body::bytes().to_owned())
}

// POST /set/[key] (BODY="value")
fn set_value(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("set" / String)
    .and(warp::post())
    .and(warp::path::end())
    .and(with_body())
    .and(with_db(db))
    .and_then(handlers::set_value)
}

// GET /get/[key]
fn get_value(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("get" / String)
    .and(warp::get())
    .and(warp::path::end())
    .and(with_db(db))
    .and_then(handlers::get_value)
}


pub(crate) fn routes(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  set_value(db.clone())
    .or(get_value(db.clone()))
}
