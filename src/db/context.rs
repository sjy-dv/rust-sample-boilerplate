use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, MySqlPool};
use std::sync::Arc;
use crate::models::base::User;

pub struct Table<'c, T>
where
    T: FromRow<'c, MySqlRow<'c>>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: fn(&MySqlRow<'c>) -> Result<T, sqlx::Error>,
}

impl<'c, T> Table<'c, T>
where
    T: FromRow<'c, MySqlRow<'c>>,
{
    fn new(pool: Arc<MySqlPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
        }
    }
}

pub struct JoinTable<'c, T1, T2>
where
    T1: FromRow<'c, MySqlRow<'c>>,
    T2: FromRow<'c, MySqlRow<'c>>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: (
        fn(&MySqlRow<'c>) -> Result<T1, sqlx::Error>,
        fn(&MySqlRow<'c>) -> Result<T2, sqlx::Error>,
    ),
}

impl<'c, T1, T2> JoinTable<'c, T1, T2>
where
    T1: FromRow<'c, MySqlRow<'c>>,
    T2: FromRow<'c, MySqlRow<'c>>,
{
    fn new(pool: Arc<MySqlPool>) -> Self {
        JoinTable {
            pool,
            _from_row: (T1::from_row, T2::from_row),
        }
    }
}


pub struct Database<'c> {
    pub users: Arc<Table<'c, User>>,
}

impl Database<'_> {
    pub async fn new(sql_url: &str) -> Database<'_> {
        let pool = MySqlPool::new(sql_url).await.unwrap();
        let pool = Arc::new(pool);

        Database {
            users: Arc::from(Table::new(pool.clone())),
        }
    }
}