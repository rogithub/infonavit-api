#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::request::LenientForm;
use infonavit_api::cors::CORS;
use infonavit_api::info::CreditInfo;
use infonavit_api::types::{Credit, Payment};
use rocket_contrib::json::Json;
use rocket::response::status;

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

#[post("/pauments", format = "application/json", data = "<payment>")]
fn create_payment(payment: LenientForm<Payment>) -> status::Accepted<()> {
    let info = CreditInfo::new("./db/infonavit.db");
    info.save_payment(payment.0);
    status::Accepted(None)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![credit, payments,create_payment])
        .attach(CORS)
        .launch();
}
