use rusqlite::{types::Value, Connection, NO_PARAMS};
use std::path::PathBuf;
use tdn_types::primitive::{new_io_error, Result};

pub enum DsValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl DsValue {
    pub fn is_none(&self) -> bool {
        match self {
            DsValue::Null => true,
            _ => false,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            DsValue::Text(s) => &s,
            _ => "",
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            DsValue::Integer(i) => i == &1i64,
            _ => false,
        }
    }

    pub fn as_i64(self) -> i64 {
        match self {
            DsValue::Integer(i) => i,
            _ => 0,
        }
    }

    pub fn as_string(self) -> String {
        match self {
            DsValue::Text(s) => s,
            _ => "".to_owned(),
        }
    }

    pub fn as_f64(self) -> f64 {
        match self {
            DsValue::Real(f) => f,
            _ => 0.0,
        }
    }

    pub fn as_vec(self) -> Vec<u8> {
        match self {
            DsValue::Blob(v) => v,
            _ => vec![],
        }
    }
}

impl From<Value> for DsValue {
    fn from(value: Value) -> DsValue {
        match value {
            Value::Null => DsValue::Null,
            Value::Integer(i) => DsValue::Integer(i),
            Value::Real(i) => DsValue::Real(i),
            Value::Text(s) => DsValue::Text(s),
            Value::Blob(v) => DsValue::Blob(v),
        }
    }
}

pub struct DStorage {
    connect: Connection,
}

impl DStorage {
    #[inline]
    pub fn open(path: PathBuf) -> Result<Self> {
        let connect =
            Connection::open(path).map_err(|_e| new_io_error("database open failure."))?;
        Ok(DStorage { connect })
    }

    /// tmp use.
    #[inline]
    pub fn execute(&self, sql: &str) -> Result<usize> {
        self.connect
            .execute(sql, NO_PARAMS)
            .map_err(|_e| new_io_error("database execute failure."))
    }

    /// tmp use.
    #[inline]
    pub fn query(&self, sql: &str) -> Result<Vec<Vec<DsValue>>> {
        let mut stmt = self
            .connect
            .prepare(sql)
            .map_err(|_e| new_io_error("database prepare failure."))?;
        let mut rows = stmt
            .query(NO_PARAMS)
            .map_err(|_e| new_io_error("database query failure."))?;

        let mut matrix: Vec<Vec<DsValue>> = Vec::new();
        while let Some(row) = rows
            .next()
            .map_err(|_e| new_io_error("database query failure."))?
        {
            let mut values: Vec<DsValue> = Vec::new();
            for i in 0..row.column_count() {
                values.push(
                    row.get::<usize, Value>(i)
                        .map_err(|_e| new_io_error("database query row failure."))?
                        .into(),
                );
            }
            matrix.push(values);
        }

        Ok(matrix)
    }

    /// tmp use. return the insert id.
    #[inline]
    pub fn insert(&self, sql: &str) -> Result<i64> {
        self.execute(sql)?;
        Ok(self.connect.last_insert_rowid())
    }

    /// tmp use.
    #[inline]
    pub fn update(&self, sql: &str) -> Result<usize> {
        self.execute(sql)
    }

    /// tmp use.
    #[inline]
    pub fn delete(&self, sql: &str) -> Result<usize> {
        self.execute(sql)
    }

    /// tmp use.
    #[inline]
    pub fn flush(&self) -> Result<()> {
        self.connect.flush_prepared_statement_cache();
        Ok(())
    }

    /// tmp use.
    #[inline]
    pub fn close(self) -> Result<()> {
        self.flush()?;
        self.connect
            .close()
            .map_err(|_e| new_io_error("database close failure."))
    }
}
