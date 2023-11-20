use rocket::routes;
use rocket_dyn_templates::Template;

mod data;
mod handlers;
mod models;
mod utils;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![handlers::index, handlers::register, handlers::login])
        .attach(Template::fairing())
}
