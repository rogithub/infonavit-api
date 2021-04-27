use chrono::Utc;
use chrono::DateTime;
use chrono::NaiveDateTime;
use crate::db_traits::{ConnBuilder, Db, DbConn};
use crate::types::Credit;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexModel {
    pub credit: Credit,
    pub credit_start_date: String
}

pub struct CreditInfo {
    db: Db,
}

impl CreditInfo {
    pub fn new(db_path: &str) -> CreditInfo {
        let db = ConnBuilder::default()
            .set_path(db_path)
            .build()
            .expect("Faild to create connection");

        CreditInfo { db }
    }

    pub fn build(&self, id: &str) -> IndexModel {
        let it = self
            .db
            .exec(&|conn| Credit::find_by_id(&conn, id))
            .expect("Faild to retrieve credit");

        let credit = match it {
            Some(c) => c,
            None => panic!("Credit not found!"),
        };

        let naive_datetime = NaiveDateTime::from_timestamp(credit.start_date, 0);
        let datetime_again: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);

        IndexModel {
            credit: credit,
            credit_start_date: datetime_again.format("%d %b %Y").to_string()
        }
        
    }
}
