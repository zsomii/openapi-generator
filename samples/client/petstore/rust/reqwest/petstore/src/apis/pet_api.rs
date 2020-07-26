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


/// struct for typed errors of method `add_pet`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddPetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_pet`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `find_pets_by_status`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindPetsByStatusError {
    DefaultResponse(Vec<crate::models::Pet>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `find_pets_by_tags`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindPetsByTagsError {
    DefaultResponse(Vec<crate::models::Pet>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_pet_by_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPetByIdError {
    DefaultResponse(crate::models::Pet),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_pet`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_pet_with_form`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePetWithFormError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `upload_file`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadFileError {
    DefaultResponse(crate::models::ApiResponse),
    UnknownValue(serde_json::Value),
}


pub fn add_pet(configuration: &configuration::Configuration, body: crate::models::Pet) -> Result<(), Error<AddPetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/pet", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&body);

    let req = req_builder.build()?;
    let mut resp = client.execute(req)?;

    let status = resp.status();
    let content = resp.text()?;

    if status.is_success() {
        Ok(())
    } else {
        let entity: Option<AddPetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

pub fn delete_pet(configuration: &configuration::Configuration, pet_id: i64, api_key: Option<&str>) -> Result<(), Error<DeletePetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/pet/{petId}", configuration.base_path, petId=pet_id);
    let mut req_builder = client.delete(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = api_key {
        req_builder = req_builder.header("api_key", param_value.to_string());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let mut resp = client.execute(req)?;

    let status = resp.status();
    let content = resp.text()?;

    if status.is_success() {
        Ok(())
    } else {
        let entity: Option<DeletePetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Multiple status values can be provided with comma separated strings
pub fn find_pets_by_status(configuration: &configuration::Configuration, status: Vec<String>) -> Result<Vec<crate::models::Pet>, Error<FindPetsByStatusError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/pet/findByStatus", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    req_builder = req_builder.query(&[("status", &status.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let mut resp = client.execute(req)?;

    let status = resp.status();
    let content = resp.text()?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<FindPetsByStatusError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Multiple tags can be provided with comma separated strings. Use tag1, tag2, tag3 for testing.
pub fn find_pets_by_tags(configuration: &configuration::Configuration, tags: Vec<String>) -> Result<Vec<crate::models::Pet>, Error<FindPetsByTagsError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/pet/findByTags", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    req_builder = req_builder.query(&[("tags", &tags.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let mut resp = client.execute(req)?;

    let status = resp.status();
    let content = resp.text()?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<FindPetsByTagsError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Returns a single pet
pub fn get_pet_by_id(configuration: &configuration::Configuration, pet_id: i64) -> Result<crate::models::Pet, Error<GetPetByIdError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/pet/{petId}", configuration.base_path, petId=pet_id);
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
    let mut resp = client.execute(req)?;

    let status = resp.status();
    let content = resp.text()?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetPetByIdError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

pub fn update_pet(configuration: &configuration::Configuration, body: crate::models::Pet) -> Result<(), Error<UpdatePetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/pet", configuration.base_path);
    let mut req_builder = client.put(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&body);

    let req = req_builder.build()?;
    let mut resp = client.execute(req)?;

    let status = resp.status();
    let content = resp.text()?;

    if status.is_success() {
        Ok(())
    } else {
        let entity: Option<UpdatePetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

pub fn update_pet_with_form(configuration: &configuration::Configuration, pet_id: i64, name: Option<&str>, status: Option<&str>) -> Result<(), Error<UpdatePetWithFormError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/pet/{petId}", configuration.base_path, petId=pet_id);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form_params = std::collections::HashMap::new();
    if let Some(param_value) = name {
        form_params.insert("name", param_value.to_string());
    }
    if let Some(param_value) = status {
        form_params.insert("status", param_value.to_string());
    }
    req_builder = req_builder.form(&form_params);

    let req = req_builder.build()?;
    let mut resp = client.execute(req)?;

    let status = resp.status();
    let content = resp.text()?;

    if status.is_success() {
        Ok(())
    } else {
        let entity: Option<UpdatePetWithFormError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

pub fn upload_file(configuration: &configuration::Configuration, pet_id: i64, additional_metadata: Option<&str>, file: Option<std::path::PathBuf>) -> Result<crate::models::ApiResponse, Error<UploadFileError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/pet/{petId}/uploadImage", configuration.base_path, petId=pet_id);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form = reqwest::multipart::Form::new();
    if let Some(param_value) = additional_metadata {
        form = form.text("additionalMetadata", param_value.to_string());
    }
    if let Some(param_value) = file {
        form = form.file("file", param_value)?;
    }
    req_builder = req_builder.multipart(form);

    let req = req_builder.build()?;
    let mut resp = client.execute(req)?;

    let status = resp.status();
    let content = resp.text()?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<UploadFileError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

