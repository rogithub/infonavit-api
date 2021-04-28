use crate::db_traits::StructRow;
use rusqlite::{params, Connection, Error, Result};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: u64,
    pub document: Vec<u8>,
    pub file_name: String,
    pub created_date: i64,
}


impl Document {
    pub fn find_by_id(conn: &Connection, id: &str) -> Result<Option<Document>, Error> {
        let sql = "SELECT id,document,file_name,created_date FROM documents WHERE id = :id ORDER BY ROWID ASC LIMIT 1;";
        let mut stmt = conn.prepare(sql)?;
        let mut item_iter = stmt.query_map(&[(":id", id)], |row| {
            Ok(Document {
                id: row.get(0)?,
                document: row.get(1)?,
                file_name: row.get(2)?,
                created_date: row.get(3)?,
            })
        })?;

        let found = item_iter.next();
        match found {
            None => Ok(None),
            Some(result) => result.and_then(|credit| Ok(Some(credit))),
        }
    }
}

impl StructRow for Document {
    fn insert(&self, conn: &Connection) -> Result<usize, Error> {
        let sql = "INSERT INTO documents (document,file_name,created_date) VALUES (?1,?2,?3);";

        conn.execute(
            sql,
            params![self.document, self.file_name, self.created_date],
        )
    }

    fn update(&self, conn: &Connection) -> Result<usize, Error> {
        let sql =
            "UPDATE documents SET document = ?1, file_name = ?2, created_date = ?3 WHERE id = ?4";

        conn.execute(
            sql,
            params![self.document, self.file_name, self.created_date, self.id],
        )
    }

    fn delete(&self, conn: &Connection) -> Result<usize, Error> {
        let sql = "DELETE documents WHERE id = ?1";
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
    fn insert_retrieve_by_id_document() {
        let date = Utc.ymd(2020, 12, 30).and_hms(12, 0, 0);
        let db = init_bd();
        let d = Document {
            id: 0,
            document: vec![1, 2, 3],
            file_name: "Payment.pfd".to_string(),
            created_date: date.timestamp(),
        };
        db.insert(&d).unwrap();
        let it = db.exec(&|conn| Document::find_by_id(&conn, "1")).unwrap();

        match it {
            Some(Document {
                id,
                document,
                file_name,
                created_date,
            }) => {
                assert_eq!(id, 1);
                assert_eq!(document, d.document);
                assert_eq!(file_name, d.file_name);
                assert_eq!(created_date, d.created_date);
            }
            _ => panic!(),
        };
    }
}
