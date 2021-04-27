use rocket_contrib::templates::tera::{from_value, to_value, Error, GlobalFn, Value};
use std::collections::BTreeMap;

pub fn utc_str_to_date(urls: BTreeMap<String, String>) -> GlobalFn {
    Box::new(move |args| -> Result<Value, Error> {
        match args.get("name") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => Ok(to_value(urls.get(&v).unwrap()).unwrap()),
                Err(_) => Err("Oops".into()),
            },
            None => Err("Oops".into()),
        }
    })
}
