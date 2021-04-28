#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use infonavit_api::cors::CORS;
use infonavit_api::info::CreditInfo;
use infonavit_api::types::{Credit, Payment};
use rocket_contrib::json::Json;

#[get("/credit/<id>")]
fn credit(id: usize) -> Json<Option<Credit>> {
    let info = CreditInfo::new("./db/infonavit.db");
    let it = info.get_credit(&id.to_string());
    Json(it)
}

#[get("/payments/<credit_id>")]
fn payments(credit_id: usize) -> Json<Vec<Payment>> {
    let info = CreditInfo::new("./db/infonavit.db");
    let it = info.get_payments(&credit_id.to_string());
    Json(it)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![credit, payments])
        .attach(CORS)
        .launch();
}
