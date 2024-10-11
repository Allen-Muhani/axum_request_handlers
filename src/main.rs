use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct UserQuery {
    address: String,
    age: u32,
}

#[derive(Serialize, Deserialize)]
struct Tx {
    amount: u128,
    address: String,
}

#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/get_transaction/:transaction_id",
            get(get_transatcion_by_id),
        )
        .route("/pass_query_params_generic", get(use_query_params))
        .route(
            "/pass_query_params_predefined",
            get(pass_query_params_predefined),
        )
        .route("/use_request_headers", get(use_request_headers))
        .route("/send_money", post(send_money))
        .route("/send_money_generic", post(send_money_generic));

    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/**
 * Gets a transaction given its ID in the database.
 * @param transaction_id the transaction id.
 */
async fn get_transatcion_by_id(Path(transaction_id): Path<u32>) -> String {
    format!("Id is {}", transaction_id)
}

/**
 * Gets data from query parameters.
 */
async fn use_query_params(Query(params): Query<HashMap<String, String>>) -> String {
    let keyid = match params.get("keyid") {
        Some(value) => value,
        None => "No keyid provided",
    };

    format!("The key id of x {}", keyid)
}

/**
 * Gets data from query parameters.
 */
async fn pass_query_params_predefined(Query(params): Query<UserQuery>) -> String {
    format!(
        "The {} year old user with address {}",
        params.age, params.address
    )
}

/**
 * Use request header from HTTP request.
 */
async fn use_request_headers(header: HeaderMap) -> String {
    let sms_header = header
        .get("sms")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("No sms header provided");

    format!("The header value for key SMS is {}", sms_header)
}

async fn send_money(Json(payload): Json<Tx>) -> Json<Tx> {
    let amount = payload.amount;
    let address = payload.address;

    let tx: Tx = Tx { amount: amount, address: address };

    Json(tx)
}

async fn send_money_generic(Json(payload): Json<Value>) -> String {
    if let Some(value) = payload.get("address") {
        // Check if the value is a string
        if let Some(string_value) = value.as_str() {
            // Do something with the string_value
            format!("The value is: {}", string_value)
        } else {
            format!("The value is not a string.")
        }
    } else {
        format!("Key not found in the payload.")
    }
}
