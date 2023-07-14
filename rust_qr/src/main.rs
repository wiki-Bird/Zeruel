
// basic rocket app that handles a get index, get /<short-url> and post /

#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};
use rocket::form::Form;

#[derive(FromForm)]
struct Url {
    big_url: String,
}

#[get("/")]
async fn index() -> Option<NamedFile> { // Display the index.html file
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/<short_url>")]
fn get_big_url(short_url: String) -> &'static str {
    // Get the short url from database
    // Redirect to the long url
    // If the short url is not found, return a 404
    "short url"
}

#[post("/?<big_url>")]
fn post_url(big_url: &str) -> String {
    // Generate a short url
    // Save the short url and long url to database
    // Return the short url
    "big url"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, get_big_url, post_url])
        .mount("/static", rocket::fs::FileServer::from("static"))
        // .mount("/static", FileServer::from("static"))
        // .attach(rocket::fairing::AdHoc::on_attach("Database Config", |rocket| {
        //     let db_url = rocket.config().get_string("db_url").unwrap();
        //     let pool = db::init_pool(db_url).expect("db pool");
        //     Ok(rocket.manage(pool))
        // }))
}