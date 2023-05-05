use std::io;
use reqwest::Error;
use serde_json::Value;
use tokio;

const BASE_URL: &str = "https://www.sastaticket.pk";

async fn fetch(url: &str) -> Result<Value, Error> {
    let response = reqwest::get(url).await?.json::<Value>().await?;
    Ok(response)
}

async fn fetch_stop(search_string: &str) -> Result<Value, Error> {
    let url = format!("{}/api/v4/air/get_all_stops/?contains={}", BASE_URL, search_string);
    let response = fetch(&url).await?;
    Ok(response["data"].clone())
}

#[tokio::main]
async fn main() {
    println!("Welcome!");
    println!("Please input your origin.");
    let mut origin = String::new();

    let mut selected_origin_airport: String = String::new();
    let mut selected_destination_airport: String = String::new();


    let mut origin_results_map = std::collections::HashMap::new();
    let mut destination_results_map = std::collections::HashMap::new();

    io::stdin()
        .read_line(&mut origin)
        .expect("Failed to read line");

    let origin = origin.trim();

    match fetch_stop(origin).await {
        Ok(matching_origins) => {
            // dictionary to map airport IATA code to number
            let mut number = 1;
            if let Some(airports) = matching_origins.as_array() {
                println!("The following airports match your search:");
                for airport in airports {
                    let airport_name = airport["airport"].as_str().unwrap_or("Unknown");
                    let city = airport["city"].as_str().unwrap_or("Unknown");
                    let country = airport["country"].as_str().unwrap_or("Unknown");
                    let iata_code = airport["iata_code"].as_str().unwrap_or("Unknown");

                    println!("{} - {} ({}) - {}, {}", number, airport_name, iata_code, city, country);
                    println!();
                    origin_results_map.insert(number, iata_code.to_string());
                    number += 1;
                }
            println!("Please select an airport by entering its number.");

            let mut airport_number = String::new();

            io::stdin()
                .read_line(&mut airport_number)
                .expect("Failed to read line");

            selected_origin_airport = origin_results_map
                .get(&airport_number.trim().parse::<i32>().unwrap_or(0))
                .unwrap_or(&String::new())
                .clone();
            



            } else {
                println!("No airports found.");
            }
        }
        Err(err) => {
            println!("Error fetching airports: {:?}", err);
        }
    }

    println!("Please input your destination.");
    let mut destination = String::new();

    io::stdin()
        .read_line(&mut destination)
        .expect("Failed to read line");

    let destination = destination.trim();

    match fetch_stop(destination).await {
        Ok(matching_destinations) => {
            // dictionary to map airport IATA code to number
            let mut number = 1;
            if let Some(airports) = matching_destinations.as_array() {
                println!("The following airports match your search:");
                for airport in airports {
                    let airport_name = airport["airport"].as_str().unwrap_or("Unknown");
                    let city = airport["city"].as_str().unwrap_or("Unknown");
                    let country = airport["country"].as_str().unwrap_or("Unknown");
                    let iata_code = airport["iata_code"].as_str().unwrap_or("Unknown");

                    println!("{} - {} ({}) - {}, {}", number, airport_name, iata_code, city, country);
                    println!();
                    destination_results_map.insert(number, iata_code.to_string());
                    number += 1;
                }
            println!("Please select an airport by entering its number.");

            let mut airport_number = String::new();

            io::stdin()
                .read_line(&mut airport_number)
                .expect("Failed to read line");

            selected_destination_airport = destination_results_map
                .get(&airport_number.trim().parse::<i32>().unwrap_or(0))
                .unwrap_or(&String::new())
                .clone();
            

            } else {
                println!("No airports found.");
            }
        }
        Err(err) => {
            println!("Error fetching airports: {:?}", err);
        }
    }

    println!("You're flying from {} to {}.", selected_origin_airport, selected_destination_airport);
}

