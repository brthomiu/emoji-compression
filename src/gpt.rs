use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use hyper::header::AUTHORIZATION;
use reqwest::{get, Client};

// Structs for request/response objects
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageResponse {
    data: String,
    method: String,
    headers: HashMap<String, String>
}

#[derive(Debug, Serialize, Deserialize)]
struct Messages<'a> {
    messages: &'a [u8]
}

#[derive(Debug, Serialize, Deserialize)]
struct FormMap<'a> {
    model: String,
    messages: Messages,
    headers: HashMap<String, String>
}


pub async fn gpt() -> Result<(), E> {
    // Declare empty string for the API key
    let mut api_key = String::new();

    // Prompt user to enter their API key
    println!("Enter your OpenAI API key.");
    std::io::stdin()
        .read_line(&mut api_key)
        .expect("Cannot read API Key");

    // Declare empty string for the message
    let mut message = String::new();

    message = "say test".into();

    // Create map for message object
    let mut message_map = HashMap::new();
    message_map.insert("user", message);

    // Create message object
    let message_object = Message {
        role: "user".into(),
        content: message.into(),
    };

    // Create body for request
    let mut form_map = HashMap::new();
    // model, messages, temperature 
    form_map.insert("gpt-3.5-turbo", message_object, );


    // Initialize HTTP client
    let client = reqwest::Client::new();

    // POST request to OpenAPI
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header(AUTHORIZATION, api_key)
        .form()
        .send()
        .await
        .unwrap()
        .text()
        .await;

    // Log response
    println!("{:?}", res)
}
