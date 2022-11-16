use rocket::serde::json::Json;
use lib::db;
use lib::model::Movie;
#[macro_use] extern crate rocket;

  
#[get("/")]
async fn get_movies() -> Json<Option<Vec<Movie>>> {
    Json(db::read_movies().await)
}

#[get("/<title>")]
async fn get_movie(title: &str) -> Json<Option<Movie>> {
    Json(db::read_movie(title).await)
}

#[post("/", data = "<movie>")]
async fn post_movie(movie: Json<Movie>) -> Json<Option<Movie>> {
    Json(db::insert_movie(movie.0).await)
}

#[delete("/", data = "<movie>")]
async fn delete_movie(movie: Json<Movie>) -> Json<bool> {
    Json(db::remove_movie(movie.0).await)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/movies",
        routes![get_movies, get_movie, post_movie, delete_movie],
    )
}