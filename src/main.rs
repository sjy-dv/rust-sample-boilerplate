use anyhow::Ok;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate failure;
// #[macro_use]
// extern crate serde_json;

#[macro_use]
mod api;
mod models;
mod db;



#[tokio::main]
async fn main() -> anyhow::Result<()> {

    api::start().await?;
    Ok(())
}