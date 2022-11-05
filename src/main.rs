use serde_json::json;
use warp::{
    reply::{json, Json},
    Filter, Rejection,
};

#[tokio::main]
async fn main() {
    let routes = warp::path::end().and(warp::get()).and_then(index);

    println!("server is started successfully on port 3000");
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

async fn index() -> Result<Json, Rejection> {
    let data = json!({"message": "great progress ya nimer"});

    Ok(json(&data))
}
