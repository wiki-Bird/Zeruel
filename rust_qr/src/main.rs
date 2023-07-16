#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket::form::Form;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS, NON_ALPHANUMERIC};

mod schema;
mod models;

mod shorten;

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
    context.insert("short_url", short_url);

    Template::render("short_url", &context)
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
    context.insert("big_url", temp);
    Template::render("big_url", &context)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, redirect_url, post_url, submit_url])
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
}
