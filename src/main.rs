use actix_web::{web, HttpServer, App, HttpResponse};
use serde::Deserialize;
use sqlx::MySqlPool;


#[derive(Clone)]
struct AppState {
    pool: MySqlPool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const URL: &str = "mysql://root:@localhost:3306/elo";
    let pool = sqlx::mysql::MySqlPool::connect(URL).await.unwrap();
    
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let app_state = AppState { pool };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/form", web::post().to(create_contact))
            
    })
    .bind(("127.0.0.1", 4000))
    .unwrap()
    .run()
    .await

}

async fn create_contact(form: web::Json<User>, app_state: web::Data<AppState>) -> HttpResponse {
    let insert = sqlx::query("INSERT INTO user (name, email, subject, message) VALUES (?, ?, ?, ?)")
        .bind(&form.name)
        .bind(&form.email)
        .bind(&form.subject)
        .bind(&form.message)
        .execute(&app_state.pool)
        .await;
    match insert {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::BadRequest().into()
    }
}

#[derive(Deserialize)]
struct User {
    name: String,
    email: String,
    subject: String,
    message: String,
}
