use rusqlite::{Connection, Error, Result};

pub trait StructRow {
    fn insert(&self, conn: &Connection) -> Result<usize, Error>;
    fn delete(&self, conn: &Connection) -> Result<usize, Error>;
    fn update(&self, conn: &Connection) -> Result<usize, Error>;
}

pub trait DbConn {
    fn insert<T: StructRow>(&self, t: &T) -> Result<usize, Error>;
    fn delete<T: StructRow>(&self, t: &T) -> Result<usize, Error>;
    fn update<T: StructRow>(&self, t: &T) -> Result<usize, Error>;
    fn exec_non_query(&self, sql: &str) -> Result<(), Error>;
    fn exec<T>(&self, f: &dyn Fn(&Connection) -> T) -> T;
}

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn new(conn: Connection) -> Db {
        Db { conn: conn }
    }
}

impl DbConn for Db {
    fn insert<T: StructRow>(&self, t: &T) -> Result<usize, Error> {
        t.insert(&self.conn)
    }
    fn delete<T: StructRow>(&self, t: &T) -> Result<usize, Error> {
        t.delete(&self.conn)
    }
    fn update<T: StructRow>(&self, t: &T) -> Result<usize, Error> {
        t.update(&self.conn)
    }
    fn exec_non_query(&self, sql: &str) -> Result<(), Error> {
        self.conn.execute_batch(sql)
    }
    fn exec<T>(&self, f: &dyn Fn(&Connection) -> T) -> T {
        f(&self.conn)
    }
}

#[derive(Default)]
pub struct ConnBuilder {
    path: Option<String>,
}

impl ConnBuilder {
    pub fn new(path: Option<String>) -> ConnBuilder {
        ConnBuilder { path: path }
    }

    pub fn set_path(mut self, path: &str) -> ConnBuilder {
        self.path = Some(path.to_string());
        self
    }
    pub fn open_in_memory(&self) -> Result<Connection, Error> {
        let connection = Connection::open_in_memory();
        connection
    }

    pub fn open(&self, path: &str) -> Result<Connection, Error> {
        let connection = Connection::open(path);
        connection
    }

    pub fn build(self) -> Result<Db, Error> {
        let conn = match self.path {
            Some(ref path) => self.open(&path)?,
            None => self.open_in_memory()?,
        };

        Ok(Db::new(conn))
    }
}
