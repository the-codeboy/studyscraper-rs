#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::response::Redirect;

#[get("/<_path..>", rank = 2)]
fn redirect_all(_path: std::path::PathBuf) -> Redirect {
    Redirect::to("/")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(
            rocket::Config::figment()
                .merge(("address", "0.0.0.0"))
                .merge(("port", 8080)),
        )
        .mount("/", routes![redirect_all])
        .mount("/", FileServer::from("static").rank(1))
}
