use crate::db_traits::{Db, DbConn};

#[derive(Default)]
pub struct DbInit;

impl DbInit {
    pub fn create_tables(&self, db: &Db) -> std::result::Result<(), rusqlite::Error> {
        let sql = "
            CREATE TABLE IF NOT EXISTS `credit` (
                `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                `credit_number` INTEGER NOT NULL, 
                `credit_name` TEXT NOT NULL,
                `start_date` INTEGER NOT NULL, 
                `interest_rate_percent` REAL NOT NULL, 
                `montly_payment_by_employer` REAL NOT NULL,
                `montly_payment_by_employee` REAL NOT NULL, 
                `total_credit_amount` REAL NOT NULL,
                `years` INTEGER NOT NULL
            );
           
            CREATE TABLE IF NOT EXISTS `documents` ( 
                `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
                `document` BLOB NOT NULL, 
                `file_name` TEXT NOT NULL, 
                `created_date` INTEGER NOT NULL 
            );
            
            CREATE TABLE IF NOT EXISTS `payments` ( 
                `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
                `credit_id` INTEGER NOT NULL,
                `payment_date` INTEGER, 
                `number` INTEGER NOT NULL,
                `amount` REAL NOT NULL,
                `is_payment_on_time` INTEGER NOT NULL, 
                `is_via_payrol` INTEGER NOT NULL,
                `document_id` INTEGER,
                `comments` TEXT
            );            
            ";

        db.exec_non_query(sql)
    }
}
