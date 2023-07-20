// load_short.rs

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use crate::schema::urls;
use crate::models;
use crate::db_connection;


pub fn load_long_url(url: &str) -> String {
    dotenv().ok();

    let mut connection = db_connection::establish_connection();
    let id = get_id(&url, &mut connection);
    if id == -1 {
        return "URL ID not found".to_string();
    }

    let long_url = get_long_url(&id, &mut connection);
    if long_url == false.to_string() {
        return "Long URL not found".to_string();
    }
    else {
        increment_uses(&id, &mut connection);
        return long_url;
    }
}

fn get_long_url(id: &i32, conn: &mut SqliteConnection) -> String {
    // connect to SQLite database using diesel and check if short_url exists
    // return its big_url if it does, "false" if it doesn't
    let connection = conn;
    let results = urls::table
        .filter(urls::id.eq(id))
        .load::<models::Url>(connection)
        .expect("Error loading urls");

    if results.len() == 0 {
        return false.to_string();
    }

    return results[0].long_url.to_string();

}

fn increment_uses(id: &i32, conn: &mut SqliteConnection) -> bool {
    let connection = conn;
    // update the uses column in the database for the short_url

    let results = diesel::update(urls::table.find(id))
        .set(urls::uses.eq(urls::uses + 1))
        .execute(connection)
        .expect("Error updating urls");

    // if the update was successful, return true
    if results == 1 {
        return true;
    }
    return false;
}

fn get_id(short_url: &str, conn: &mut SqliteConnection) -> i32 {
    let connection = conn;
    // get the id of the short_url
    let results = urls::table
        .filter(urls::small_url.eq(short_url))
        .load::<models::Url>(connection)
        .expect("Error loading urls");

    // log the results to the console
    println!("Found {} urls with short_url {}", results.len(), short_url);

    if results.len() == 0 {
        // No record found, return None
        return -1;
    }
    // return the id
    results[0].id
}