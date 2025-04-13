// Simple Rust CLI chat with Gemini AI

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    env,
    io::{self, Read},
};

#[derive(Serialize)]
struct RequestBody {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct ResponseBody {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY env var not set!");
    let api_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let args: Vec<String> = env::args().collect();

    let mut prompt: String = "".to_string();
    if args.len() == 2
    // prompt as an arg
    {
        prompt = args[1].clone()
    } else
    // prompt as a stdin pipe
    {
        io::stdin()
            .read_to_string(&mut prompt)
            .expect("failed to read stdin");
    }

    let request_body = RequestBody {
        contents: vec![Content {
            parts: vec![Part { text: prompt }],
        }],
    };

    let response = Client::new()
        .post(&api_url)
        .json(&request_body)
        .send()
        .await?;
    if !response.status().is_success() {
        eprintln!("error: {}", response.status());
        eprintln!("response: {:?}", response.text().await?);
        return Ok(());
    }

    let response_body: ResponseBody = response.json().await?;
    if let Some(candidate) = response_body.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            println!("{}", part.text);
        } else {
            eprintln!("no response text received");
        }
    } else {
        eprintln!("no candidates in response");
    }

    Ok(())
}
