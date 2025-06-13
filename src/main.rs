#![warn(missing_docs)]

//! This crate is used to import data from the [FoodData Central](https://fdc.nal.usda.gov/fdc-app.html#/food-search) of the US Department of Agriculture
//! to a [Tandoor](https://tandoor.dev/) instance using the FDC ID of Tandoor foods to link them to their FoodData Central counterpart.  

use std::collections::HashMap;
use std::{fs, io};
use std::error::Error;
use std::sync::{Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use regex::Regex;
use reqwest::{Client};
use log::{debug, info, warn, error, trace};
use env_logger;
use clap::Parser;

mod models;
use models::configuration::Configuration;
use models::tandoor::internal_tandoor_food::InternalTandoorFood;
use models::tandoor::internal_tandoor_food_api_response::InternalTandoorFoodApiResponse;
use models::tandoor::internal_tandoor_property::InternalTandoorProperty;
use models::tandoor::internal_tandoor_food_property::InternalTandoorFoodProperty;
use models::tandoor::api_tandoor_food::ApiTandoorFood;
use models::usda::usda_food::USDAFood;
use models::usda::usda_api_response::USDAApiResponse;
use models::command_line_arguments::Args;

#[tokio::main]
async fn main(){

    // Get command line arguments.
    let args = Args::parse();
    let override_properties = args.override_properties;
    debug!("Interactive mode enabled: {}", args.interactive);
    debug!("Override mode enabled: {}", args.override_properties);

    // Create client for api requests.
    let client = Arc::new(Client::new());

    // Initialize logger (with set log level for the crate
    env_logger::Builder::new().filter(Some(env!("CARGO_PKG_NAME")), args.log_level.into()).init();

    // Read app settings
    let app_settings = fs::read_to_string("./appsettings.json").expect("The appsettings were not loaded successfully.");
    let configuration: Configuration = serde_json::from_str(&app_settings).expect("The appsettings were not well-formatted.");
    let usda_api_key = configuration.usda_api_key;
    let tandoor_endpoint = format!("http://{}/api/", configuration.tandoor_url);
    debug!("The configured Tandoor API endpoint is: {}", tandoor_endpoint);
    let tandoor_api_key = configuration.tandoor_api_key;
    let tandoor_version = configuration.tandoor_version;

    // Get Properties
    let mut tandoor_properties: Vec<InternalTandoorProperty> = Vec::new();
    match get_food_properties(&client, &tandoor_endpoint, &tandoor_api_key).await {
        Ok(props) => {
            tandoor_properties = Some(props).unwrap();
            info!("Found {} properties.", tandoor_properties.len());
            trace!("{}", serde_json::to_string(&tandoor_properties).unwrap());
        }
        Err(e) => {
            error!("Error fetching food properties: {:?}", e);
        }
    }
    let tandoor_property_id_name: HashMap<i32, String> = tandoor_properties.iter().map(|x| (x.fdc_id, x.name.to_string())).collect();

    // Get Foods
    let mut tandoor_foods: Vec<InternalTandoorFood> = Vec::new();
    match get_foods(&client, &tandoor_endpoint, &tandoor_api_key).await {
        Ok(props) => {
            tandoor_foods = Some(props).unwrap();
            info!("Found {} foods.", tandoor_foods.len());
        }
        Err(e) => {
            error!("Error fetching foods: {:?}", e);
        }
    }

    // Update the foods.
    let updated_foods = Arc::new(AtomicUsize::new(0));
    let not_updated_foods = Arc::new(AtomicUsize::new(0));
    let no_fdc_id = Arc::new(AtomicUsize::new(0));
    let already_fully_updated = Arc::new(AtomicUsize::new(0));
    let number_of_properties =  tandoor_properties.iter().count();
    let mut handles = vec![];
    for mut food in tandoor_foods.into_iter(){
        // Directly continue if number of properties of food is equal to number of properties
        // retrieved from Tandoor and override is not enabled.
        if !args.override_properties && food.properties.iter().count() == number_of_properties{
            info!("{} is already fully updated.", food.name);
            {
                already_fully_updated.fetch_add(1, Ordering::SeqCst);  // Lock the mutex to modify the shared counter
            }
            continue;
        }
        
        let client = Arc::clone(&client);
        let tandoor_property_id_name = tandoor_property_id_name.clone();
        let override_properties = override_properties.clone();
        let tandoor_endpoint = tandoor_endpoint.clone();
        let tandoor_api_key = tandoor_api_key.clone();
        let usda_api_key = usda_api_key.clone();
        let updated_foods = Arc::clone(&updated_foods);
        let not_updated_foods = Arc::clone(&not_updated_foods);
        let no_fdc_id = Arc::clone(&no_fdc_id);
        
        let handle = tokio::spawn(async move{
            debug!("Going to update food {}", food.name);
            // Get data from USDA
            let fdc_id: i32;
            if let Some(id) = get_fdc_id(&food, &args.interactive){
                debug!("Found FDC ID {} for food {}.", id, food.name);
                fdc_id = id
            }else{
                warn!("Food {} does not have a FDC ID and will not be updated.", food.name);
                {
                    no_fdc_id.fetch_add(1, Ordering::SeqCst);
                }
                return;
            }

            let usda_data = match get_food_data(&client, &fdc_id, &usda_api_key, &tandoor_property_id_name).await {
                Ok(props) => {
                    debug!("Fetched properties for food {} from the USDA FDC database using {} as the ID", food.name, fdc_id);
                    // When fetching the data was successful, override FDC ID field with the used one.
                    food.fdc_id = Some(fdc_id);
                    props
                }
                Err(e) => {
                    warn!("Error fetching food properties for {} from the FDC database: {:?}", food.name, e);
                    {
                        not_updated_foods.fetch_add(1, Ordering::SeqCst);
                    }
                    return;
                }
            };

            // Build updated food
            let (food_id, updated_food) = match create_updated_food(&food, &usda_data.food, &override_properties){
                Ok(props) => {
                    debug!("Build updated food for {}", food.name);
                    props
                }
                Err(e) => {
                    warn!("Error creating updated food for {}: {:?}", food.name, e);
                    {
                        not_updated_foods.fetch_add(1, Ordering::SeqCst);
                    }
                    return;
                }
            };

            // Update food in Tandoor database.
            let _ = match update_food(&client, &tandoor_endpoint, &tandoor_api_key, &updated_food, &food_id).await{
                Ok(_) => {
                    {
                        updated_foods.fetch_add(1, Ordering::SeqCst);
                        info!("Successfully updated food {}", updated_food.name);
                    }
                }
                Err(e) => {
                    warn!("Error updating food {}: {:?}", updated_food.name, e);
                    { 
                        not_updated_foods.fetch_add(1, Ordering::SeqCst);
                    }
                    return;
                }
            };

            // Check for USDA requests left if < 20 wait a minute before continuing.
            if usda_data.requests_left < 20 {
                let sleep_time = 60;
                info!("There are only {} requests left before being rate-limited. To prevent that the program will now sleep for {} seconds before continuing.", usda_data.requests_left, sleep_time);
                tokio::time::sleep(Duration::from_millis(sleep_time * 1000)).await;
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles{
        handle.await.expect("TODO: panic message");
    }
    
    info!("\n {} foods successfully updated. \n {} foods were not updated successfully. \
        \n {} foods did not have a FDC ID. \n {} foods were already completely updated.", 
        updated_foods.load(Ordering::SeqCst), 
        not_updated_foods.load(Ordering::SeqCst), 
        no_fdc_id.load(Ordering::SeqCst), 
        already_fully_updated.load(Ordering::SeqCst));
}

/// Gets all food properties of the Tandoor instance
/// ### Parameters
/// - client: The client used for any http requests
/// - tandoor_endpoint: The endpoint of the Tandoor instance.
/// - tandoor_api_key: The API key to interact with the Tandoor API
/// ### Returns
/// Vec containing a list of all properties that were returned by the Tandoor API.
async fn get_food_properties(client: &Client, tandoor_endpoint: &str, tandoor_api_key: &str) -> Result<Vec<InternalTandoorProperty>, Box<dyn Error>> {
    let url = if (tandoor_version == legacy) { format!("{}food-property-type/", tandoor_endpoint) } else { format!("{}property-type/", tandoor_endpoint) };
    trace!("Getting food properties by calling {}", url);
    let response = client.get(url)
        .header("Authorization", format!("Bearer {}", tandoor_api_key))
        .send()
        .await?
        .error_for_status()?;

    let body = response.text().await?;
    let properties: Vec<InternalTandoorProperty> = serde_json::from_str(&body)?;
    Ok(properties)
}

/// Gets all foods of the Tandoor instance
/// ### Parameters
/// - client: The client used for any http requests
/// - tandoor_endpoint: The endpoint of the Tandoor instance.
/// - tandoor_api_key: The API key to interact with the Tandoor API
/// ### Returns
/// Vec containing a list of all foods that were returned by the Tandoor API.
async fn get_foods(client: &Client, tandoor_endpoint: &str, tandoor_api_key: &str) -> Result<Vec<InternalTandoorFood>, Box<dyn Error>>{
    let mut url = format!("{}food/", tandoor_endpoint);
    let mut tandoor_foods: Vec<InternalTandoorFood> = Vec::new();
    let mut expected_food_number: i32;
    loop {
        trace!("Loading foods by calling {}", url);
        let response = client.get(&url)
            .header("Authorization", format!("Bearer {}", tandoor_api_key))
            .send()
            .await?
            .error_for_status()?;

        let body = response.text().await?;
        trace!("Retrieved foods from Tandoor: \n {}", body);
        let tandoor_food_api_request: InternalTandoorFoodApiResponse = serde_json::from_str(&body)?;
        tandoor_foods.extend(tandoor_food_api_request.results);
        expected_food_number = tandoor_food_api_request.count;
        if let Some(next_url) = tandoor_food_api_request.next {
            url = next_url;
            debug!("Loaded {} foods.", tandoor_foods.len())
        } else {
            break;
        }
    }

    if expected_food_number != i32::try_from(tandoor_foods.len())?{
        panic!("Not all foods were returned successfully. Please check the logs for more information. Stopping execution.");
    }
    Ok(tandoor_foods)
}

///  Gets the data of the requested food from the USDA database and filters out properties that are not wanted.
/// ### Parameters
/// - client: The client used for any http requests
/// - fdc_id: The FDC ID of the food to search for
/// - usda_api_key: The API key for the FoodData Central database
/// - tandoor_food_id_name: A HashMap containing the FDC ID of a property and its name in the Tandoor instance
/// ### Returns
/// A USDAApiResponse object representing the response or an error.
/// ### Remarks
/// As the Tandoor API requires a property that we want to add to be identified by the name of the property we need to replace the name of FDC food property 
/// with the name the user set in the Tandoor instance. Thus, we need the property name and not only the property id.
async fn get_food_data(client: &Client, fdc_id: &i32, usda_api_key: &str, tandoor_property_id_name: &HashMap<i32, String>) -> Result<USDAApiResponse, Box<dyn Error>>{

    // Ask USDA database for data using the fdc_id of the food    
    let request_url = format!("https://api.nal.usda.gov/fdc/v1/food/{}?", fdc_id);
    trace!("Getting data from FDC by calling {}", request_url);
    let response = client.get(request_url)
        .header("X-Api-Key", usda_api_key)
        .send()
        .await?
        .error_for_status()?;

    // Remember the requests we have left, so we do not get blocked.
    let requests_left: i32 = match response.headers().get("X-RateLimit-Remaining") {
        Some(value) => value.to_str().unwrap().parse().unwrap(),
        None => {
            warn!("Header indicating the requests left before being rate limited not found - Setting requests left to 0.");
            0
        }
    };
    let body = response.text().await?;
    let mut food: USDAFood = serde_json::from_str(&body)?;
    
    // Filter the properties out that we do not want
    food.food_nutrients.retain(|x| tandoor_property_id_name.contains_key(&x.nutrient_information.id));

    // Update the names of the usda food properties with the names of the tandoor properties
    for nutrient in &mut food.food_nutrients {
        if let Some(new_name) = tandoor_property_id_name.get(&nutrient.nutrient_information.id) {
            nutrient.nutrient_information.name = new_name.clone();
        }
    }

    // construct return value
    let usda_api_response = USDAApiResponse {
        requests_left,
        food
    };
    Ok(usda_api_response)
}

/// Creates the updated food object to send to Tandoor
/// ### Parameters
/// - tandoor_food: The current representation of the food as requested from the Tandoor instance.
/// - usda_food: The food with its nutrients retrieved from the FoodData Central.
/// ### Returns
/// Tuple representing the id of the food and a food item that can be sent to the Tandoor API in order to update it or an error.
fn create_updated_food(tandoor_food: &InternalTandoorFood, usda_food: &USDAFood, override_properties: &bool) -> Result<(i32, ApiTandoorFood), Box<dyn Error>>{
    let mut local_food = (*tandoor_food).clone();

    // If overriding is active, simply clear all current properties so that is_id_present below is
    // always false.
    if *override_properties{
        trace!("Deleting current properties of food {}.", local_food.name);
        local_food.properties.clear();
    }

    for usda_nutrient in usda_food.food_nutrients.iter(){
        let is_id_present = local_food.properties.iter().any(|a| {
            a.property_type.fdc_id == usda_nutrient.nutrient_information.id
        });
        if !is_id_present {
            local_food.properties.push(InternalTandoorFoodProperty::from(usda_nutrient));
            trace!("Adding property {} to food {}", usda_nutrient.nutrient_information.name, local_food.name)
        }
    }    
    Ok((tandoor_food.id, ApiTandoorFood::from(local_food)))
}

/// Updates the food in the Tandoor database
/// ### Parameters
/// - client: The client used for any http requests.
/// - tandoor_endpoint: The endpoint of the Tandoor instance.
/// - tandoor_api_key: The API key to interact with the Tandoor API
/// - food: The food data that should be sent to the API
/// - food_id: The id of the food that should be updated with the data given by 'food' parameter
/// ### Returns
/// boolean indicating success of the update or an error.
async fn update_food(client: &Client, tandoor_endpoint: &String, tandoor_api_key: &String, food: &ApiTandoorFood, food_id: &i32) -> Result<bool, Box<dyn Error>>{
    // Use given food and call Tandoor API to update food.
    let url = format!("{}food/{}/", tandoor_endpoint, food_id);
    debug!("Calling {} to update food {}", url, food.name);
    let _ = client.patch(url)
        .header("Authorization", format!("Bearer {}", tandoor_api_key))
        .json(food)
        .send()
        .await?
        .error_for_status()?;
    Ok(true)
}

/// Gets the FDC ID from a given food (either from the URL field or the FDC ID field).
/// ### Parameters
/// - food: The food for which the FDC ID should be retrieved.
/// ### Returns
/// Option<i32> containing the FDC ID or None if no FDC ID was found in the URL or FDC ID field and was not given by the user.
fn get_fdc_id(food: &InternalTandoorFood, is_interactive: &bool) -> Option<i32>{
    
    // Closure to handle fallback to FDC ID field/User input.
    let get_fdc_id_from_field = || {
        if let Some(fdc_id_of_food) = food.fdc_id.clone() {
            trace!("Found FDC ID {} in FDC ID field.", fdc_id_of_food);
            Some(fdc_id_of_food)
        } else {
            // If interactive mode is activated ask for user input, else return None.
            if *is_interactive {
                get_fdc_id_from_user_input(&food)
            }else{
                None
            }
        }
    };
    
    let re = Regex::new(r"food-details/(\d+)/nutrients").unwrap();
    // If URL is set use that to get FDC ID
    // if no URL is set or no FDC ID can be matched from the URL, use FDC ID field.
    return if let Some(food_url) = food.url.clone() {
        if let Some(caps) = re.captures(&*food_url) {
            let fdc_id = (&caps[1]).parse().unwrap();
            trace!("Found FDC ID {} in the URL field.", fdc_id);
            Some(fdc_id)
        } else {
            get_fdc_id_from_field()
        }
    } else {
        get_fdc_id_from_field()
    }
}

/// Asks the user for input of an FDC ID to update the food.
/// ### Parameters
/// - food: The food for which the FDC ID should be retrieved.
/// ### Returns
/// Option<i32> containing the FDC ID or None if the user submitted an empty response.
fn get_fdc_id_from_user_input(food: &InternalTandoorFood) -> Option<i32>{
    println!("Provide the FDC ID for food {}. Leave empty to skip the food. Submit with Enter.", food.name);
    loop{
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Error reading from stdin.");

        // Check for empty response
        let trimmed_response = response.trim();
        if trimmed_response.is_empty() {
            return None;
        }
        
        // Check if valid number was entered, otherwise ask again
        match trimmed_response.parse::<u32>() {
            Ok(i) => {
                trace!("Got FDC ID {} from user input.", i);
                return Some(i as i32)
            },
            Err(..) => {
                println!("Your input (\"{}\") was not a number, try again.", trimmed_response)
            },
        };
    }
}
