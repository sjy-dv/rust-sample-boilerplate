use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};

#[derive(Debug, Serialize, Deserialize)]
pub struct Resp {
    pub header_value: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub page: i32,
    pub name: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl<'c> FromRow<'c, MySqlRow<'c>> for User {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        })
    }
}