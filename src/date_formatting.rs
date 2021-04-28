use chrono::Utc;
use chrono::DateTime;
use chrono::NaiveDateTime;
use rocket_contrib::templates::tera::{from_value, to_value, Error, GlobalFn, Value};
use std::collections::BTreeMap;

pub fn format_date(urls: BTreeMap<String, String>) -> GlobalFn {
    Box::new(move |args| -> Result<Value, Error> {
        match args.get("date") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => {

                    

                    let number = urls.get(&v).unwrap()
                    .parse::<i64>().unwrap();                    
                    
                    let naive_datetime = NaiveDateTime::from_timestamp(number, 0);
                    let datetime_again: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc); 
                    
                    Ok(to_value(datetime_again.format("%d %b %Y").to_string()).unwrap())
                },
                Err(_) => Err("Oops 1".into()),
            },
            None => Err("Oops 2".into()),
        }
    })
}
