use std::env;
use dotenv::dotenv;
use reqwest;
use serde::Deserialize;
use tokio;

/// Weather in your terminal, powered by Rust!

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

// Async function to fetch weather data
async fn fetch_weather(api_key: String, city: String) -> Result<(), reqwest::Error> {
    // Build the URL for the OpenWeatherMap API request
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=imperial",
        city.trim(),
        api_key
    );
    // Make the API request and handle errors
    let response = reqwest::get(&url).await?;
    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        // Parse the JSON response into our WeatherData struct
        let weather_data: WeatherData = response.json().await?;
        // Extract and print relevant weather information
        let temperature = weather_data.main.temp;
        let description = &weather_data.weather[0].description;
        println!("Weather in {}: {:.2}°F, {}", city.trim(), temperature, description);
    } else {
        // Print an error message if the request was not successful
        println!("Error: {}", response.status());
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    // Load variables from .env file
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let mut city = String::new();
    println!("Enter a city name: ");
    std::io::stdin().read_line(&mut city).expect("Failed to read line");

    let city_clone = city.clone();

    // Spawn a Tokio task to run the asynchronous fetch_weather function
    tokio::spawn(fetch_weather(api_key, city_clone));
    // Sleep to keep the program running while the asynchronous task completes
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

}
