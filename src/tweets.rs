use actix_web::{get, post, delete, HttpResponse};
use actix_web::web::Path;

// api/tweets
#[get("/tweets")]
pub async fn get_tweets() -> HttpResponse {
    // obtener tweets
    let tweets = ["tweet 1: Hola", "tweet 2: Adios"];
    HttpResponse::Ok().content_type("application/json").json(tweets)
}

// create tweet
#[post("/tweets")]
pub async fn create_tweet() -> HttpResponse {
    let new_tweet = "Este es un uevo tweet";
    HttpResponse::Created().content_type("application/json").json(new_tweet)
}

// get tweet
#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(path: Path<(String,)>) -> HttpResponse {
    let tweet = format!("Este es el tweet {:?}", path.0);
    HttpResponse::Ok().content_type("application/json").json(tweet)
}

//cargo install diesel_cli --no-default-features --features postgres