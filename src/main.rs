#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use infonavit_api::info::CreditInfo;
use infonavit_api::types::{Credit, Payment};
use rocket::http::Method;
use rocket::http::Status;
use rocket::Response;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::error::Error;

#[get("/credit/<id>")]
fn credit(id: usize) -> Json<Option<Credit>> {
    let info = CreditInfo::new("./db/infonavit.db");
    let it = info.get_credit(&id.to_string());
    Json(it)
}

#[get("/credit/<credit_id>/payments")]
fn payments(credit_id: usize) -> Json<Vec<Payment>> {
    let info = CreditInfo::new("./db/infonavit.db");
    let it = info.get_payments(&credit_id.to_string());
    Json(it)
}

#[post("/payment", format = "application/json", data = "<payment>")]
fn create_payment<'a>(payment: Json<Payment>) -> Response<'a> {
    let info = CreditInfo::new("./db/infonavit.db");
    info.save_payment(payment.0);
    let mut res = Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let allowed_origins = AllowedOrigins::All;

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .mount("/", routes![credit, payments, create_payment])
        .attach(cors)
        .launch();

    Ok(())
}
