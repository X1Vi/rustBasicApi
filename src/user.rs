
use rocket::serde::json::Json;
use serde::Serialize;
use rand::Rng;
use crate::models::Post;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}
#[get("/")]
pub fn random_array() -> Json<[i32; 4]> {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Generate an array of 4 random numbers between 1 and 100 (inclusive)
    let random_arr = [
        rng.gen_range(1..=100),
        rng.gen_range(1..=100),
        rng.gen_range(1..=100),
        rng.gen_range(1..=100),
    ];

    // Return the array as a JSON response
    Json(random_arr)
}

#[post("/")]
pub fn CreatePost() {
    
}