#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
pub mod schema; 
mod user;
mod models;
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![user::index])
        .mount("/user/randomArray", routes![user::random_array])
        
}