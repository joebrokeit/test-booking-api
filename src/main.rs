

use std::net::SocketAddr;
use axum::{Router, routing::{get, post}, response::Json};
use serde::{Deserialize, Serialize};
use axum::http::StatusCode;

#[derive(Serialize)]
struct GetTicketsNumb {
    tickets_number: i32,
}

#[derive(Serialize)]
struct PostTicketResponse {
    message: String,
    ticket: i32,
}

#[derive(Deserialize)]
struct PostTickets {
    ticket: i32,
}

// get handler to return number of tickets
async fn get_tickets() -> Json<GetTicketsNumb> {
    Json(GetTicketsNumb {
        tickets_number: 34,
    })
}

// post handler to post tickets
async fn post_tickets(Json(tickets): Json<PostTickets>) -> (StatusCode, Json<PostTicketResponse>) {
    let respmsg = PostTicketResponse {
        message: "Ticket created successfully".to_string(),
        ticket: tickets.ticket,
    };
    (StatusCode::CREATED, Json(respmsg))
}

#[tokio::main]
async fn main() {
    // a router for both post and get request
    let booking_system = Router::new()
        .route("/ticket", get(get_tickets))
        .route("/tickets", post(post_tickets));
    let address = SocketAddr::from(([0, 0, 0, 0], 7000));
    // running the server on localhost and port 7000
    println!("Running on http://{}", address);
    axum::Server::bind(&address)
        .serve(booking_system.into_make_service())
        .await
        .unwrap();
}
