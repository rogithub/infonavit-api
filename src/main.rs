#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket_contrib::templates::Template;

use std::collections::HashMap;

#[get("/")]
fn index() -> Template {
    let mut context: HashMap<String, i32> = HashMap::new();
    context.insert("test".to_string(), 1);
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
