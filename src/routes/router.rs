use axum ::{routing::get, Router};

pub fn create_routes() ->Router{
    Router::new().route("/",get(home))
}


async fn home () -> &'static str{
    "Rust server is running"
}

