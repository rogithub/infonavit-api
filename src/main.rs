#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use infonavit_web::info::CreditInfo;
use rocket_contrib::templates::Template;

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
        .launch();
}
