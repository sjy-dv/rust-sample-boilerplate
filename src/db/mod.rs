use crate::models::base::User;


pub mod context;
mod user;


pub type Database<'c> = context::Database<'c>;
pub type Table<'c, T> = context::Table<'c, T>;
pub type JoinTable<'c, T1, T2> = context::JoinTable<'c, T1, T2>;