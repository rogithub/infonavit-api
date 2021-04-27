#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use infonavit_web::date_formatting::utc_str_to_date;
use infonavit_web::info::CreditInfo;
use rocket_contrib::templates::Template;
use std::collections::BTreeMap;

#[get("/")]
fn index() -> Template {
    let info = CreditInfo::new("./db/infonavit.db");
    let context = info.get_credit("1");
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .attach(Template::custom(|engines| {
            let url = BTreeMap::new();
            engines
                .tera
                .register_function("utc_str_to_date", utc_str_to_date(url))
        }))
        .launch();
}
