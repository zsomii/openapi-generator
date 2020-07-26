/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed successes of method `delete_order`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOrderSuccess {
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `get_inventory`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInventorySuccess {
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `get_order_by_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrderByIdSuccess {
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `place_order`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlaceOrderSuccess {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_order`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOrderError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_inventory`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInventoryError {
    DefaultResponse(::std::collections::HashMap<String, i32>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_order_by_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrderByIdError {
    DefaultResponse(crate::models::Order),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `place_order`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlaceOrderError {
    DefaultResponse(crate::models::Order),
    UnknownValue(serde_json::Value),
}


/// For valid response try integer IDs with value < 1000. Anything above 1000 or nonintegers will generate API errors
pub async fn delete_order(configuration: &configuration::Configuration, order_id: &str) -> Result<ResponseContent<DeleteOrderSuccess>, Error<DeleteOrderError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/store/order/{orderId}", configuration.base_path, orderId=crate::apis::urlencode(order_id));
    let mut req_builder = client.delete(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        let entity: Option<DeleteOrderSuccess> = serde_json::from_str(&content).ok();
        let result = ResponseContent { status, content, entity };
        Ok(result)
    } else {
        let entity: Option<DeleteOrderError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Returns a map of status codes to quantities
pub async fn get_inventory(configuration: &configuration::Configuration, ) -> Result<ResponseContent<GetInventorySuccess>, Error<GetInventoryError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/store/inventory", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let val = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("api_key", val);
    };

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        let entity: Option<GetInventorySuccess> = serde_json::from_str(&content).ok();
        let result = ResponseContent { status, content, entity };
        Ok(result)
    } else {
        let entity: Option<GetInventoryError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// For valid response try integer IDs with value <= 5 or > 10. Other values will generated exceptions
pub async fn get_order_by_id(configuration: &configuration::Configuration, order_id: i64) -> Result<ResponseContent<GetOrderByIdSuccess>, Error<GetOrderByIdError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/store/order/{orderId}", configuration.base_path, orderId=order_id);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        let entity: Option<GetOrderByIdSuccess> = serde_json::from_str(&content).ok();
        let result = ResponseContent { status, content, entity };
        Ok(result)
    } else {
        let entity: Option<GetOrderByIdError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

pub async fn place_order(configuration: &configuration::Configuration, body: crate::models::Order) -> Result<ResponseContent<PlaceOrderSuccess>, Error<PlaceOrderError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/store/order", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&body);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        let entity: Option<PlaceOrderSuccess> = serde_json::from_str(&content).ok();
        let result = ResponseContent { status, content, entity };
        Ok(result)
    } else {
        let entity: Option<PlaceOrderError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

