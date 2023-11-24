use rocket::routes;
use rocket_dyn_templates::Template;
use crate::handlers;

pub fn server() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![handlers::index, handlers::register, handlers::login, handlers::dashboard])
        .attach(Template::fairing())
}
