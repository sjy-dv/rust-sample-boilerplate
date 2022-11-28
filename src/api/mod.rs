use actix_web::{
    web::{Query},
    web,
    App, HttpRequest,
    HttpServer,
    HttpResponse, Responder,
};

use serde_json::json;
use std::env;
use std::sync::{Arc, Mutex};
use crate::models::base;
use crate::db::Database;

pub struct AppState<'a> {
    pub connections: Mutex<u32>,
    pub context: Arc<Database<'a>>,
}


async fn hello(query:Query<base::Info>,req : HttpRequest) -> HttpResponse {
    let auth = req.headers().get("Authorization").unwrap().to_str().unwrap();
    let mut resp = base::Resp{header_value:String::from("")};
    println!("{:?}",query);
    resp.header_value = format!("{}", auth);
    return HttpResponse::Ok().json(resp);
}

async fn get_user(user_id: web::Path<String>, app_state: web::Data<AppState<'_>>) -> impl Responder{
    let user = app_state.context.users.get_user_by_id(&user_id).await;
    match user {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(user) => {
            HttpResponse::Ok().json(user)
        }
    }
}


pub async fn start() -> anyhow::Result<()>{

    let database_url = env::var("db").expect("DATABASE_URL must be set");

    let db_context = Database::new(&database_url).await;

    let app_state = web::Data::new(AppState{
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    });

    HttpServer::new(move|| {
        App::new().app_data(app_state.clone()).configure(routes)
    })
    .bind(("0.0.0.0", 8081))?
    .run().await?;
    println!("server is running!");
    Ok(())
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/").to(hello));
    app.service(web::resource("/{id}").route(web::get().to(get_user)));
}