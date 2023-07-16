// shorten.rs

use rand::Rng;


pub fn shorten_url(url: &str) -> String {
    let rand_string = rand_string();

    return rand_string;
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