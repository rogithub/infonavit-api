use crate::db_traits::StructRow;
use rusqlite::{params, Connection, Error, Result};
use crate::types::{Credit};

impl Credit {
    pub fn find_by_id(conn: &Connection, id: &str) -> Result<Option<Credit>, Error> {
        let sql = "SELECT id,credit_number,credit_name,start_date,interest_rate_percent,montly_payment_by_employer,montly_payment_by_employee,total_credit_amount,years FROM credit WHERE id = :id ORDER BY ROWID ASC LIMIT 1;";
        let mut stmt = conn.prepare(sql)?;
        let mut item_iter = stmt.query_map(&[(":id", id)], |row| {
            Ok(Credit {
                id: row.get(0)?,
                credit_number: row.get(1)?,
                credit_name: row.get(2)?,
                start_date: row.get(3)?,
                interest_rate_percent: row.get(4)?,
                montly_payment_by_employer: row.get(5)?,
                montly_payment_by_employee: row.get(6)?,
                total_credit_amount: row.get(7)?,
                years: row.get(8)?,
            })
        })?;

        let found = item_iter.next();
        match found {
            None => Ok(None),
            Some(result) => result.and_then(|credit| Ok(Some(credit))),
        }
    }
}

impl StructRow for Credit {
    fn insert(&self, conn: &Connection) -> Result<usize, Error> {
        let sql = "INSERT INTO credit (credit_number,credit_name,start_date,interest_rate_percent,montly_payment_by_employer,montly_payment_by_employee,total_credit_amount,years) VALUES (?1,?2,?3,?4,?5,?6,?7,?8)";

        conn.execute(
            sql,
            params![
                self.credit_number,
                self.credit_name,
                self.start_date,
                self.interest_rate_percent,
                self.montly_payment_by_employer,
                self.montly_payment_by_employee,
                self.total_credit_amount,
                self.years
            ],
        )
    }

    fn update(&self, conn: &Connection) -> Result<usize, Error> {
        let sql = "UPDATE credit SET credit_number = ?1, credit_name = ?2, start_date = ?3, interest_rate_percent = ?4, montly_payment_by_employer = ?5, montly_payment_by_employee = ?6, total_credit_amount = ?7 WHERE id = ?8";

        conn.execute(
            sql,
            params![
                self.credit_number,
                self.credit_name,
                self.start_date,
                self.interest_rate_percent,
                self.montly_payment_by_employer,
                self.montly_payment_by_employee,
                self.total_credit_amount,
                self.id
            ],
        )
    }

    fn delete(&self, conn: &Connection) -> Result<usize, Error> {
        let sql = "DELETE credit WHERE id = ?1";
        conn.execute(sql, params![self.id])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db_init::DbInit;
    use crate::db_traits::{ConnBuilder, Db, DbConn};
    use chrono::TimeZone;
    use chrono::Utc;
    fn init_bd() -> Db {
        let db = ConnBuilder::default().build().unwrap();
        let seed = DbInit::default();
        seed.create_tables(&db).unwrap();
        db
    }

    #[test]
    fn insert_retrieve_by_id_credit() {
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
        let it = db.exec(&|conn| Credit::find_by_id(&conn, "1")).unwrap();

        match it {
            Some(Credit {
                id,
                credit_number,
                credit_name,
                start_date,
                interest_rate_percent,
                montly_payment_by_employer,
                montly_payment_by_employee,
                total_credit_amount,
                years,
            }) => {
                assert_eq!(id, 1);
                assert_eq!(credit_number, c.credit_number);
                assert_eq!(credit_name, c.credit_name);
                assert_eq!(start_date, c.start_date);
                assert_eq!(interest_rate_percent, c.interest_rate_percent);
                assert_eq!(montly_payment_by_employer, c.montly_payment_by_employer);
                assert_eq!(montly_payment_by_employee, c.montly_payment_by_employee);
                assert_eq!(total_credit_amount, c.total_credit_amount);
                assert_eq!(years, c.years);
            }
            _ => panic!(),
        };
    }
}
