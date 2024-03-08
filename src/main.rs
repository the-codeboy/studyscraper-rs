#[macro_use]
extern crate rocket;

use regex::Regex;
use rocket::fs::FileServer;
use rocket::response::Redirect;
use reqwest::header::{HeaderMap, HeaderValue};

#[get("/download/<url>")]
async fn download(url: &str) -> Redirect {
    if !url.starts_with("https://www.studydrive.net/") {
        return Redirect::to("/not_found");
    }
    let doc_id = url
        .split("/")
        .last()
        .unwrap()
        .split("?")
        .collect::<Vec<_>>()[0];

    println!("doc_id={}", doc_id);

    let body = ureq::get(url).call().unwrap().into_string().unwrap();
    let re = Regex::new(r##"token=(.*?)""##).unwrap();
    let Some(caps) = re.captures(&*body) else {
        println!("no token found");
        let json = send_get_request(doc_id).await.unwrap();
        let token = get_token().await.unwrap();
        let data = json["data"].as_object().unwrap();
        let name = data["filename"].as_str().unwrap();
        let ending=name.split(".")
            .last()
            .unwrap();
        return Redirect::to(format!("https://cdn.studydrive.net/d/prod/documents/{}/original/{}.{}?token={}", doc_id, doc_id,ending, token));
    };
    let token = &caps[1];
    Redirect::to(format!("https://cdn.studydrive.net/d/prod/documents/{}/original/{}.pdf?token={}", doc_id, doc_id, token))
}

async fn get_token() -> Result<String, Box<dyn std::error::Error>> {
    let doc_id= "1617040";
    let json = send_get_request(doc_id).await?;
    
    let data = json["data"].as_object().unwrap();
    let preview = data["file_preview"].as_str().unwrap();
    let token = preview.split("token=")
        .last()
        .unwrap();
    Ok(token.to_string())
}

async fn send_get_request(doc_id: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>>{
    let url = format!("https://www.studydrive.net/document/{}", doc_id);
    let mut headers = HeaderMap::new();
    headers.insert("X-Requested-With", HeaderValue::from_static("XMLHttpRequest"));
    let client = reqwest::Client::new();
    let response = client.get(&url).headers(headers).send().await.expect("Failed to send request");

    // handle the response as json
    let json = response.json::<serde_json::Value>().await.expect("Failed to parse json");
    
    Ok(json)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(
            rocket::Config::figment()
                .merge(("address", "0.0.0.0"))
                .merge(("port", 8080)),
        )
        .mount("/", routes![download])
        .mount("/", FileServer::from("static"))
}
