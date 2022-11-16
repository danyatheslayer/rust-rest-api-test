use rocket::tokio::fs;
use crate::model::Movie;

static MOVIES_DB: &str = "./data/movies.json";

async fn _movies() -> Result<Vec<Movie>, serde_json::Error> {
    let data = fs::read_to_string(MOVIES_DB).await.expect("Error reading file");
    let movies: Result<Vec<Movie>, serde_json::Error> = serde_json::from_str(&data);
    movies
}

async fn _write_movies(movies: Vec<Movie>) { //optional using json from file
    let data = serde_json::to_string(&movies).expect("Failed to serialize");
    fs::write(MOVIES_DB, data).await.expect("Failed to write");
}

pub async fn read_movies() -> Option<Vec<Movie>> {
    match _movies().await {
        Ok(movies) => Some(movies),
        Err(_) => None
    }
}

pub async fn read_movie(title: &str) -> Option<Movie> {
    match _movies().await {
        Ok(movies) => {
            let mut iter = movies.into_iter();
            iter.find(|x| x.title == title)
        },
        Err(_) => None
    }
}

pub async fn insert_movie(movie: Movie) -> Option<Movie> {
    match _movies().await {
        Ok(mut movies) => {
            movies.push(movie.clone());
            _write_movies(movies).await;
            Some(movie)
        },
        Err(_) => None
    }
}

pub async fn remove_movie(movie: Movie) -> bool {
    match _movies().await {
        Ok(mut movies) => {
            let index = movies
                .iter()
                .position(|x| x.title == movie.title && x.genre == movie.genre);

                match index {
                    Some(x) => {
                        movies.remove(x);
                        _write_movies(movies).await;
                        true
                    } 
                    None => false
                }
        },
        Err(_) => false
    }
}
