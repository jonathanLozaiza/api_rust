use actix_web::{web, App, HttpServer, HttpResponse, Responder};

mod tweets;
mod likes;

async fn saludar() -> impl Responder {
    HttpResponse::Ok().body("HOLA!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/hola", web::get()
        .to(saludar))
        .service(tweets::get_tweets)
        .service(tweets::get_tweet_by_id)
        .service(tweets::create_tweet)
        .service(likes::get_likes_by_tweet)
        .service(likes::like_tweet)
        .service(likes::remove_like)
    }).bind("127.0.0.1:8000")?.run().await
}
