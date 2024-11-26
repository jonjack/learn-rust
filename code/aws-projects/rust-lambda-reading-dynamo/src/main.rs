use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_dynamodb::operation::get_item::GetItemOutput;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_runtime::{run, service_fn, tracing, Error as LambdaError, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    tracing::init_default_subscriber();
    run(service_fn(handler)).await?;
    Ok(())
}

#[derive(Deserialize)]
struct CustomEvent {
    first_name: String,
    last_name: String,
}

fn convert_item_to_json(get_item_output: GetItemOutput) -> Option<JsonValue> {
    // Match on the item field of GetItemOutput
    match get_item_output.item {
        Some(item_map) => {
            // Convert the HashMap<String, AttributeValue> to JSON
            Some(convert_attribute_map_to_json(item_map))
        }
        None => None,
    }
}

fn convert_attribute_map_to_json(item_map: HashMap<String, AttributeValue>) -> JsonValue {
    // Create a new JSON object
    let mut json_map = serde_json::Map::new();

    // Iterate through each key-value pair in the HashMap
    for (key, value) in item_map {
        json_map.insert(key, attribute_value_to_json(value));
    }

    JsonValue::Object(json_map)
}

fn attribute_value_to_json(attr: AttributeValue) -> JsonValue {
    match attr {
        AttributeValue::S(s) => JsonValue::String(s),
        AttributeValue::N(n) => {
            if let Ok(num) = n.parse::<f64>() {
                JsonValue::Number(serde_json::Number::from_f64(num).unwrap()) // TODO: replace unwrap as it panics
            } else {
                JsonValue::String(n)
            }
        }
        AttributeValue::Bool(b) => JsonValue::Bool(b),
        AttributeValue::L(list) => {
            JsonValue::Array(list.into_iter().map(attribute_value_to_json).collect())
        }
        AttributeValue::M(map) => {
            let mut json_map = serde_json::Map::new();
            for (k, v) in map {
                json_map.insert(k, attribute_value_to_json(v));
            }
            JsonValue::Object(json_map)
        }
        AttributeValue::Null(_) => JsonValue::Null,
        AttributeValue::Ss(set) => {
            JsonValue::Array(set.into_iter().map(JsonValue::String).collect())
        }
        AttributeValue::Ns(set) => JsonValue::Array(
            set.into_iter()
                .filter_map(|n| {
                    n.parse::<f64>()
                        .ok()
                        .and_then(|num| serde_json::Number::from_f64(num))
                        .map(JsonValue::Number)
                })
                .collect(),
        ),
        _ => JsonValue::Null, // Handle other types as needed
    }
}

async fn handler(event: LambdaEvent<CustomEvent>) -> Result<JsonValue, LambdaError> {
    let uuid = Uuid::new_v4().to_string();
    let first_name = event.payload.first_name;
    let last_name = event.payload.last_name;

    tracing::info!("first_name: {first_name}");
    tracing::info!("last_name: {last_name}");

    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");
    let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&config);

    let write_request = client
        .put_item()
        .table_name("users")
        .item("uid", AttributeValue::S(String::from(&uuid)))
        .item("first_name", AttributeValue::S(String::from(first_name)))
        .item("last_name", AttributeValue::S(String::from(last_name)));

    write_request.send().await?;

    tracing::info!("Record with UUID {uuid} written ok");

    // Since we have the partition key (uuid we created above) we can do a getItem
    let read_request: aws_sdk_dynamodb::operation::get_item::builders::GetItemFluentBuilder =
        client
            .get_item()
            .table_name("users")
            .key("uid", AttributeValue::S(String::from(&uuid)));

    let get_item_output = read_request.send().await?;

    match convert_item_to_json(get_item_output) {
        Some(json_value) => {
            println!("Item as JSON: {}", json_value);
            // You can also convert to a specific type using serde
            // let typed_item: YourType = serde_json::from_value(json_value)?;
        }
        None => println!("Item not found"),
    }

    Ok(().into())

    //tracing::info!("item: {item:?}");

    //Ok(json!({ "message": "Record written & then read" }))
}
