use crate::db_traits::{ConnBuilder, Db, DbConn};
use crate::types::{Credit, Payment};

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

    pub fn get_credit(&self, id: &str) -> Option<Credit> {
        let it = self
            .db
            .exec(&|conn| Credit::find_by_id(&conn, id))
            .expect("Faild to retrieve credit");

        it
    }

    pub fn get_payments(&self, credit_id: &str) -> Vec<Payment> {
        let it = self
            .db
            .exec(&|conn| Payment::for_credit(&conn, credit_id))
            .expect("Faild to retrieve credit");
        it
    }
}
