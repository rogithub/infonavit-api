use crate::db_traits::StructRow;
use crate::types::Payment;
use rusqlite::{params, Connection, Error, Result};

impl Payment {
    pub fn find_by_id(conn: &Connection, id: &str) -> Result<Option<Payment>, Error> {
        let sql = "SELECT id,credit_id,payment_date,number,amount,is_payment_on_time,is_via_payrol,document_id,comments FROM payments WHERE id = :id ORDER BY ROWID ASC LIMIT 1;";
        let mut stmt = conn.prepare(sql)?;
        let mut item_iter = stmt.query_map(&[(":id", id)], |row| {
            Ok(Payment {
                id: row.get(0)?,
                credit_id: row.get(1)?,
                payment_date: row.get(2)?,
                number: row.get(3)?,
                amount: row.get(4)?,
                is_payment_on_time: row.get(5)?,
                is_via_payrol: row.get(6)?,
                document_id: row.get(7)?,
                comments: row.get(8)?,
            })
        })?;

        let found = item_iter.next();
        match found {
            None => Ok(None),
            Some(result) => result.and_then(|credit| Ok(Some(credit))),
        }
    }
}

impl StructRow for Payment {
    fn insert(&self, conn: &Connection) -> Result<usize, Error> {
        let sql = "INSERT INTO payments (credit_id,payment_date,number,amount,is_payment_on_time,is_via_payrol,document_id,comments) VALUES (?1,?2,?3,?4,?5,?6,?7,?8)";

        conn.execute(
            sql,
            params![
                self.credit_id,
                self.payment_date,
                self.number,
                self.amount,
                self.is_payment_on_time,
                self.is_via_payrol,
                self.document_id,
                self.comments
            ],
        )
    }

    fn update(&self, conn: &Connection) -> Result<usize, Error> {
        let sql = "UPDATE payments SET credit_id = ?1, payment_date = ?2, number = ?3, amount = ?4, is_payment_on_time = ?5, is_via_payrol = ?6, document_id = ?7, comments=?8 WHERE id = ?9";

        conn.execute(
            sql,
            params![
                self.credit_id,
                self.payment_date,
                self.number,
                self.amount,
                self.is_payment_on_time,
                self.is_via_payrol,
                self.document_id,
                self.comments,
                self.id
            ],
        )
    }

    fn delete(&self, conn: &Connection) -> Result<usize, Error> {
        let sql = "DELETE payments WHERE id = ?1";
        conn.execute(sql, params![self.id])
    }
}

#[cfg(test)]
mod test {
    use crate::db_init::DbInit;
    use crate::db_traits::{ConnBuilder, Db, DbConn};
    use crate::types::{Credit, Payment};
    use chrono::TimeZone;
    use chrono::Utc;
    fn init_bd() -> Db {
        let db = ConnBuilder::default().build().unwrap();
        let seed = DbInit::default();
        seed.create_tables(&db).unwrap();
        db
    }

    fn insert_credit() {
        let date = Utc.ymd(2020, 12, 30).and_hms(12, 0, 0);
        let db = init_bd();
        let c = Credit {
            id: 0,
            credit_number: 2320190039,
            credit_name: "Palmas Turquesa".to_string(),
            start_date: date.timestamp(),
            interest_rate_percent: 12.0,
            montly_payment_by_employer: 4_825.10,
            montly_payment_by_employee: 5_485.59,
            total_credit_amount: 504_494.10,
            years: 30,
        };
        db.insert(&c).unwrap();
    }

    #[test]
    fn insert_retrieve_by_id_payment() {
        let date = Utc.ymd(2020, 12, 30).and_hms(12, 0, 0);
        let db = init_bd();
        insert_credit();

        let p = Payment {
            id: 0,
            credit_id: 1,
            payment_date: date.timestamp(),
            number: 2,
            is_payment_on_time: true,
            is_via_payrol: false,
            amount: 10_000.10,
            document_id: None,
            comments: "hello".to_string(),
        };
        db.insert(&p).unwrap();
        let it = db.exec(&|conn| Payment::find_by_id(&conn, "1")).unwrap();

        match it {
            Some(Payment {
                id,
                credit_id,
                payment_date,
                number,
                is_payment_on_time,
                is_via_payrol,
                amount,
                document_id,
                comments,
            }) => {
                assert_eq!(id, 1);
                assert_eq!(credit_id, 1);
                assert_eq!(payment_date, p.payment_date);
                assert_eq!(number, p.number);
                assert_eq!(is_payment_on_time, p.is_payment_on_time);
                assert_eq!(is_via_payrol, p.is_via_payrol);
                assert_eq!(amount, p.amount);
                assert_eq!(document_id, p.document_id);
                assert_eq!(comments, p.comments);
            }
            _ => panic!(),
        };
    }
}
