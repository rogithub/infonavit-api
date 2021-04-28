#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use infonavit_api::cors::CORS;
use infonavit_api::info::CreditInfo;
use infonavit_api::types::Credit;
use rocket_contrib::json::Json;

#[get("/credit/<id>")]
fn index(id: usize) -> Json<Option<Credit>> {
    let info = CreditInfo::new("./db/infonavit.db");
    let it = info.get_credit(&id.to_string());
    Json(it)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(CORS)
        .launch();
}
