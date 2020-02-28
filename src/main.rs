use bson::{bson, doc};
use chrono::prelude::*;
use mongodb::{options::ReplaceOptions, Client};
use reqwest;
use std::env;
//locals
use in_sight::*;

mod in_sight;

const URI: &str = "https://api.nasa.gov/insight_weather/?api_key=DEMO_KEY&feedtype=json&ver=1.0";

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let connection_string = match env::var("MONGODB_URI") {
        Ok(connection_string) => connection_string,
        Err(e) => {
            println!("Failed to get MONGODB_URI environment variable \n {}; \n Using mongodb://localhost:27018", e);
            String::from("mongodb://localhost:27018")
        }
    };
    
    let client = Client::with_uri_str(&connection_string)?;
    let db = client.database("mars_weather");
    let raw_responses = db.collection("raw_responses");
    let valid_sols = db.collection("valid_sols");

    let resp = reqwest::blocking::get(URI)?.text()?;
    
    // connect to db, write earliest date
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

    let mut sol = bson::to_bson(
        in_sight_response
            .earliest_valid_sol()
            .expect("Failed to obtain earliest valid Sol"),
    )?
    .as_document()
    .expect("Failed to convert to BSON document")
    .clone();

    sol.insert("sol_date", earliest_valid_sol_date.clone());

    valid_sols.replace_one(
        doc! {
            "sol_date": earliest_valid_sol_date
        },
        sol,
        ReplaceOptions::builder().upsert(true).build(),
    )?;

    Ok(())
}
