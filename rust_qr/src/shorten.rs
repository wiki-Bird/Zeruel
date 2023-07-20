// shorten.rs

use rand::Rng;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use crate::schema::urls;
use crate::models;
use crate::db_connection;


pub fn shorten_url(url: &str) -> String {
    dotenv().ok();
    let mut rand_url = rand_string();
    let base_url = env::var("CURRENT_URL") // get url from .env file
        .expect("CURRENT_URL must be set");
    let short_url = format!("{}", rand_url);

    // connect to SQLite database using diesel and check if rand_string exists as a short_url
    let mut connection = db_connection::establish_connection();
    let mut exists = check_short(&short_url, &mut connection);
    while exists {
        rand_url = rand_string();
        let short_url = format!("{}", rand_url);
        exists = check_short(&short_url, &mut connection);
    }

    if update_db(&short_url, &url, &mut connection) {
        return base_url + "/" + &short_url;
    }
    else {
        panic!("Error updating database");
    }
}

fn rand_string() -> String {
    // generate a random 6 character string using rand
    let rand_string: String = rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    return rand_string;
}

fn check_short(short_url: &str, conn: &mut SqliteConnection) -> bool {
    // connect to SQLite database using diesel and check if short_url exists
    // return true if it does, false if it doesn't
    let connection = conn;
    // check if short_url exists in database
    let results = urls::table
        .filter(urls::small_url.eq(short_url))
        .load::<models::Url>(connection)
        .expect("Error loading urls");

    if results.len() == 0 {
        return false;
    }

    return true;
}

fn update_db(short_url: &str, long_url: &str, conn: &mut SqliteConnection) -> bool {
    // connect to SQLite database using diesel and insert the new short_url and long_url
    let connection = conn;
    let new_url = models::NewUrl {
        small_url: short_url,
        long_url,
    };
    

    diesel::insert_into(urls::table)
        .values(&new_url)
        .execute(connection)
        .expect("Error saving new url");

    return true;
}
