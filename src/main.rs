// use std::io;

// fn main() {
//     // Create Axum server with the following endpoints:
//     // 1. GET /movie/{id} - This should return back a movie given the id
//     // 2. POST /movie - this should save movie in a DB (HashMap<String, Movie>). This movie will be sent
//     // via a JSON payload. 
    
//     // As a bonus: implement a caching layer so we don't need to make expensive "DB" lookups, etc.
    
//     struct Movie {
//         id: String,
//         name: String,
//         year: u16,
//         was_good: bool
//     }
// }


// fn main() {
//     println!("Hello, world!");
// }

use warp::{http, Filter};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use serde::Serialize;
use serde::Deserialize;

type Movies = HashMap<String, Movie>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool
}
// struct Item {
//     name: String,
//     quantity: i32,
// }

#[derive(Clone)]
struct Store {
  movie_list: Arc<RwLock<Moives>>
}

impl Store {
    fn new() -> Self {
        Store {
            movie_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}


#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_movies = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("movie"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_movie_list);

    let get_movies = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("movie"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_movie_list);

    let routes = add_movies.or(get_movies);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn get_movie_list (
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let result = store.movie_list.read();
        Ok(warp::reply::json(&*result))
}

fn post_json() -> impl Filter<Extract = (Movie,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

// id: String,
// name: String,
// year: u16,
// was_good: bool

async fn update_movie_list(
    movie: Movie,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.movie_list.write().insert(movie.id, movie.name, movie.year, movie.was_good);
        Ok(warp::reply::with_status(
            "Added movie to the movie list",
            http::StatusCode::CREATED,
        ))
}
