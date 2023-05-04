use dotenv::dotenv;
use firebase_rs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
    age: u8,
    email: String,
    id: String,
}

#[tokio::main]
async fn main() {
    let user = Person {
        name: "david".to_string(),
        age: 23,
        email: "david@gmail.com".to_string(),
    };

    // Load the .env file
    dotenv().ok();

    // Get the Firebase URL from an environment variable
    let url = env::var("FIREBASE_DATABASE_URI")
        .expect("FIREBASE_DATABASE_URI environment variable not set");

    // Create a new Firebase client using the URL
    let firebase = Firebase::new(&url).unwrap();

    let response = set_user(&firebase, &user).await;

    let user_found = get_user(&firebase, &response.id).await;

    println!("One user{:?}", user_found);

    let users = get_users(&firebase).await;

    println!("All users {:?}", users);
}

async fn set_user(firebase_client: &Firebase, user: &Person) -> Response {
    let firebase = firebase_client.at("persons");
    let _user = firebase.set::<Person>(&user).await;
    return string_to_response(&_user.unwrap().data);
}

async fn get_user(firebase: &Firebase, id: &str) -> Person {
    let firebase = firebase.at("persons").at(&id);
    let user = firebase.get::<Person>().await;
    user.unwrap()
}

async fn get_users(client: &Firebase) -> HashMap<String, Person> {
    let firebase = client.at("persons");
    let users = firebase.get::<HashMap<String, Person>>().await;
    return users.unwrap();
}

/// Convert a string to a Response struct
fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}
