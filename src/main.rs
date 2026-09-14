use std::env;
use dotenv::dotenv;
use reqwest;
use serde::Deserialize;
use tokio;

/// Weather in your terminal, powered by Rust!
#[derive(Deserialize)]
struct Coordinates {
    lat: f64,
    lon: f64,
    name: String,
    country: String,
    zip: String
}
#[derive(Deserialize)]
struct WeatherData {
    main: Main,
    weather: Vec<Weather>,
}
#[derive(Deserialize)]
struct Main {
    temp: f64,
}
#[derive(Deserialize)]
struct Weather {
    description: String
}

async fn fetch_coords_by_zipcode(api_key: &str, zipcode: &str) -> Result<Coordinates, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/geo/1.0/zip?zip={}&appid={}",
        zipcode.trim(),
        api_key
    );
    reqwest::get(&url).await?.json().await
}

// Async function to fetch weather data
async fn fetch_weather(api_key: &str, lat: f64, lon: f64) -> Result<WeatherData, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=imperial",
        lat,
        lon,
        api_key
    );
    reqwest::get(&url).await?.json().await
}

#[tokio::main]
async fn main() {
    // Load variables from .env file
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    
    let mut zipcode = String::new();
    println!("Enter a zipcode: ");
    std::io::stdin().read_line(&mut zipcode).expect("Failed to read line");

    let coords = match fetch_coords_by_zipcode(&api_key, &zipcode).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("fetch_coords_by_zipcode failed: {}", e);
            return;
        }
    };
    println!("{},{} ({}) has a lat of: {} and a lon of: {}", coords.name, coords.country, coords.zip, coords.lat, coords.lon);

    match fetch_weather(&api_key, coords.lat, coords.lon).await {
        Ok(w) => println!("Weather in {}: {:.2}°F, {}", coords.name, w.main.temp, w.weather[0].description),
        Err(e) => eprintln!("fetch_weather failed: {}", e),
    }
}
