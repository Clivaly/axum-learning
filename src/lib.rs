mod routes;

use routes::create_routes;

// run() is a function to run the server on port 3000
pub async fn run() {
    // here we get the routes
    let app = create_routes();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
