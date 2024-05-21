use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{routing::post, Router};

mod db;
mod query_engine;
mod user_service;

use db::DB; // Import DB

async fn graphql_handler(graphql_request: GraphQLRequest) -> GraphQLResponse {
    let query = query_engine::Query { db: DB }; // Creating an Query Instance

    let schema = Schema::new(query, EmptyMutation, EmptySubscription);

    let res = schema.execute(graphql_request.into_inner()).await;

    res.into()
}

#[tokio::main]
async fn main() {
    // Our API GraphQL in Rust
    let app = Router::new().route("/gql", post(graphql_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening....!");
    axum::serve(listener, app).await.unwrap();
}
