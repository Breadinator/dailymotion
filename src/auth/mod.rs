use oauth2::{
    basic::{
        BasicClient,
        BasicTokenType,
    },
    StandardTokenResponse,
    EmptyExtraTokenFields,
    ClientId,
    ClientSecret,
    AuthUrl,
    TokenUrl,
    RedirectUrl,
};
use crate::{
    Result,
    EnvironmentVariable,
};

mod listener;
use listener::listen;

pub const AUTH_URL: &str = "https://www.dailymotion.com/oauth/authorize";
pub const TOKEN_URL: &str = "https://api.dailymotion.com/oauth/token";

/// Listens for OAuth credentials.
///
/// # Errors
/// * [`TcpListener`] fails to bind
pub fn get(oauth_client: &BasicClient) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
    listen("localhost:8080", oauth_client)
}

/// Listens for OAuth credentials using environment variables.
///
/// # Errors
/// * See [`get`]
/// * If it can't get the environment variable values (see [`EnvironmentVariable`])
///
/// # Panics
/// * If it can't parse [`AUTH_URL`] or [`TOKEN_URL`]
/// * If <http://localhost:8080> is, for whatever reason, considered invalid (it shouldn't be)
pub fn get_using_env() -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
    let client = BasicClient::new(
        ClientId::new(EnvironmentVariable::DailymotionClientId.get_value()?),
        Some(ClientSecret::new(EnvironmentVariable::DailymotionClientSecret.get_value()?)),
        AuthUrl::new(AUTH_URL.to_owned()).unwrap(), // TODO: remove unwrap
        Some(TokenUrl::new(TOKEN_URL.to_owned()).unwrap()), // TODO: remove unwrap
    )
    .set_redirect_uri(
        RedirectUrl::new(String::from("http://localhost:8080"))
            .expect("Invalid redirect URL")
    );

    get(&client)
}

