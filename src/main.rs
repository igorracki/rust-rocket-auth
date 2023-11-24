mod data;
mod handlers;
mod models;
mod utils;
mod server;

#[rocket::launch]
pub fn rocket() -> _ {
    server::server()
}
