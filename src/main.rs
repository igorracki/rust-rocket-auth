use lazy_static::lazy_static;
use rocket::routes;
use rocket_dyn_templates::Template;

mod handlers;
mod models;
mod utils;
mod data;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![handlers::index, handlers::register, handlers::login])
        .attach(Template::fairing())
}
