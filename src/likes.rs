use actix_web::{get, post, delete, HttpResponse};
use actix_web::web::Path;

// get likes by tweet
#[get("/tweets/{id}/likes")]
pub async fn get_likes_by_tweet(path: Path<(String,)>) -> HttpResponse {
    let likes = 100;
    HttpResponse::Ok().content_type("application/json").json(likes)
}

// like tweet
#[post("/tweets/{id}/likes")]
pub async fn like_tweet(path: Path<(String,)>) -> HttpResponse {
    let like = "Ok";
    HttpResponse::Created().content_type("application/json").json(like)
}

// remove like
#[delete("/tweets/{id}/likes")]
pub async fn remove_like(path: Path<(String,)>) -> HttpResponse {
    HttpResponse::NoContent().content_type("application/json").await.unwrap()
}