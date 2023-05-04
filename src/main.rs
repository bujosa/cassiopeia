use std::collections::HashMap;
use std::env;

use firebase_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
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
    let user = User {
        name: "david".to_string(),
        age: 23,
        email: "david@gmail.com".to_string(),
    };

    // Get the Firebase URL from an environment variable
    let url = env::var("FIREBASE_DATABASE_URI")
        .expect("FIREBASE_DATABASE_URI environment variable not set");

    // Create a new Firebase client using the URL
    let firebase = Firebase::new(&url).unwrap();

    let response = set_user(&firebase, &user).await;

    // let user = get_user(&firebase, &response.id).await;

    // println!("{:?}", user);

    // let users = get_users(&firebase).await;

    // println!("{:?}", users);
}

async fn set_user(client: &Firebase, user: &User) -> Response {
    let firebase = client.at("users");

    let _user = firebase.set::<User>(user).await;

    string_to_response(&_user.unwrap().data)
}

// async fn get_user(firebase: &Firebase, id: &str) -> User {
//     let firebase = firebase.at("users").at(&id);
//     let user = firebase.get::<User>().await;
//     user.unwrap()
// }

// async fn get_users(client: &Firebase) -> HashMap<String, User> {
//     let firebase = client.at("users");
//     let users = firebase.get::<HashMap<String, User>>().await;
//     return users.unwrap();
// }

/// Convert a string to a Response struct
fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}
