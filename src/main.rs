#[macro_use]
extern crate rocket;

use regex::Regex;
use rocket::fs::FileServer;
use rocket::response::Redirect;

#[get("/download/<url>")]
fn hello(url: &str) -> Redirect {
    if !url.starts_with("https://www.studydrive.net/") {
        return Redirect::to("/not_found");
    }
    let body = ureq::get(url).call().unwrap().into_string().unwrap();
    let re = Regex::new(r##"token=(.*?)""##).unwrap();
    let Some(caps) = re.captures(&*body) else {
        println!("no match!");
        return Redirect::to("/not_found");
    };
    let token = &caps[1];
    let doc_id = url.split("/").last().unwrap().split("?").collect::<Vec<_>>()[0];

    println!("doc_id={}", doc_id);
    Redirect::to(format!("https://cdn.studydrive.net/d/prod/documents/{}/original/{}.pdf?token={}", doc_id, doc_id, token))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8080)))
        .mount("/", routes![hello])
        .mount("/", FileServer::from("static"))
}
