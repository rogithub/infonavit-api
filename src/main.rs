#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use std::collections::BTreeMap;
use infonavit_web::info::CreditInfo;
use rocket_contrib::templates::Template;
use infonavit_web::date_formatting::format_date;

#[get("/")]
fn index() -> Template {
    let info = CreditInfo::new("./db/infonavit.db");
    let context = info.build("1");
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::custom(|engines|{
            let url = BTreeMap::new();
            engines
                .tera
                .register_function("format_date", format_date(url))
        }))        
        .launch();
}
