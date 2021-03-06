use bson::doc;
use chrono::prelude::*;

#[cfg(feature = "lambda")]
use lambda_runtime::{error::HandlerError, lambda, Context};

#[cfg(feature = "lambda")]
use serde_json::Value;

use log;

use mongodb::{
    options::ReplaceOptions,
    sync::Client,
};

use reqwest;
use serde::Serialize;
use simple_logger;
use std::env;
//locals
use in_sight::*;

mod in_sight;

#[derive(Serialize, Clone)]
struct Outcome {
    message: String,
}

const URI: &str = "https://api.nasa.gov/insight_weather/?api_key=DEMO_KEY&feedtype=json&ver=1.0";
const MONGO_DEFAULT_URI: &str = "mongodb://localhost:27017";

fn fetch_and_store() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let connection_string = match env::var("MONGODB_URI") {
        Ok(connection_string) => connection_string,
        Err(e) => {
            println!("Failed to get MONGODB_URI environment variable \n {}; \n Using {}", e, MONGO_DEFAULT_URI);
            MONGO_DEFAULT_URI.to_owned()
        }
    };

    let client = Client::with_uri_str(&connection_string)?;
    let db = client.database("mars_weather");
    let raw_responses = db.collection("raw_responses");
    let valid_sols = db.collection("valid_sols");

    let resp = reqwest::blocking::get(URI)?.text()?;

    raw_responses.insert_one(
        doc! {
            "createdAt": Utc::now().timestamp().to_string(),
            "rawInSight": &resp.to_owned()
        },
        None,
    )?;
    
    let in_sight_response: InSight = serde_json::from_str(&resp)?;

    let earliest_valid_sol_date = in_sight_response
        .earliest_valid_sol_date()
        .expect("Failed to obtain earliest valid Sol date");

    let sol = bson::to_bson(
        &in_sight_response
            .earliest_valid_sol()
            .expect("Failed to obtain earliest valid Sol"),
    )?
    .as_document()
    .expect("Failed to convert to BSON document")
    .clone();

    valid_sols.replace_one(
        doc! {
            "sol_date": earliest_valid_sol_date.clone()
        },
        sol,
        ReplaceOptions::builder().upsert(true).build(),
    )?;

    Ok(earliest_valid_sol_date)
}

#[cfg(feature = "lambda")]
fn handler(_: Value, _: Context) -> Result<Outcome, HandlerError> {
    match fetch_and_store() {
        Ok(earliest_valid_sol_date) => Ok(Outcome {
            message: format!("Successfully stored {}", earliest_valid_sol_date),
        }),
        Err(error) => {
            log::error!("Failed to store sol data: {}", error);
            Err(HandlerError::from("Fail"))
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;
    
    #[cfg(feature = "lambda")]
    lambda!(handler);

    #[cfg(not(feature = "lambda"))]
    fetch_and_store()?;

    Ok(())
}
