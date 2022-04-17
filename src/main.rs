mod db;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
  let db = db::Db::new();
  let allowed_routes = routes::routes(db);
  
  warp::serve(allowed_routes)
    .run(([127, 0, 0, 1], 8000))
    .await;
}
