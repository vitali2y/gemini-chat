use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The prompt to send to Gemini AI
    prompt: String,
}

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

#[derive(Deserialize, Debug)]
struct ResponseBody {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize, Debug)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize, Debug)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize, Debug)]
struct ResponsePart {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let args = Args::parse();

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY env var not set!");
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let request_body = RequestBody {
        contents: vec![Content {
            parts: vec![Part { text: args.prompt }],
        }],
    };

    let response = Client::new().post(&url).json(&request_body).send().await?;
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
