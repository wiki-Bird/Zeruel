#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket::form::Form;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC}; //AsciiSet, CONTROLS, 

mod schema;
mod models;
mod shorten;
mod load_short;
mod db_connection;

#[derive(FromForm)]
pub struct UrlForm {
    big_url: String,
}


#[get("/")]
fn index() -> Template {
    Template::render("index", &HashMap::<String, String>::new())
}

#[get("/<short_url>")]
fn redirect_url(short_url: String) -> Template {
    let mut context = HashMap::new();

    let long_url = load_short::load_long_url(&short_url);
    if long_url == "URL ID not found".to_string() || long_url == "Long URL not found".to_string() {
        context.insert("error", long_url);
        return Template::render("error", &context);
    }

    context.insert("big_url", long_url);
    Template::render("big_url", &context)
}

#[post("/", data = "<url_form>")]
fn post_url(url_form: Form<UrlForm>) -> Redirect {
    // decode the URL so we can use it in the URI
    let decoded_url = utf8_percent_encode(&url_form.big_url, NON_ALPHANUMERIC).to_string();
    // redirect to the submit page
    Redirect::to(format!("/submit/{}", decoded_url))
}

#[get("/submit/<url>")]
fn submit_url(url: String) -> Template {

    let temp = shorten::shorten_url(&url);

    let mut context = HashMap::new();
    context.insert("short_url", temp);
    Template::render("short_url", &context)
}


#[catch(404)]
fn not_found() -> Template {
    let mut context = HashMap::new();
    context.insert("error", "404 Not Found".to_string());
    Template::render("error", &context)
}
#[catch(500)]
fn internal_error() -> Template {
    let mut context = HashMap::new();
    context.insert("error", "500 Internal Server Error".to_string());
    Template::render("error", &context)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, redirect_url, post_url, submit_url])
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
        .register("/", catchers![not_found, internal_error])
}
