use crate::{
    endpoint,
    api::{
        Endpoint,
        EndpointParams,
        HttpRequestMethod,
    },
};
use serde::{Serialize, Deserialize, de::DeserializeOwned};

// pub static GET: Endpoint<()> = endpoint!("/videos", HttpRequestMethod::Get, false, &[]);

pub static GET_FROM_ID: Endpoint<GetFromIdParams, GetFromIdResponse> = endpoint!("/video/{id}", HttpRequestMethod::Get, false, &[]);

pub struct GetFromIdParams {
    pub id: String,
    pub fields: Option<Vec<VideoFields>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetFromIdResponse {
    pub id: String,
    pub title: String,
    pub channel: String,
    pub owner: String,
}

impl EndpointParams for GetFromIdParams {
    fn build_url<T: EndpointParams, P: DeserializeOwned>(&self, endpoint: &Endpoint<T, P>) -> String {
        endpoint.url.to_owned().replace("{id}", &self.id)
    }
}

#[allow(clippy::module_name_repetitions)]
pub enum VideoFields {}

