use std::{
    net::{
        TcpListener,
        TcpStream,
    },
    io::{
        BufReader,
        BufRead,
        Write,
    },
};
use url::Url;
use oauth2::{
    AuthorizationCode,
    // CsrfToken,
    StandardTokenResponse,
    EmptyExtraTokenFields,
    basic::{
        BasicClient,
        BasicTokenType,
    },
    reqwest::http_client,
};
use crate::{
    Error,
    Result,
};

pub fn listen(listen_url: &str, oauth_client: &BasicClient) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
    let listener = TcpListener::bind(listen_url)?;

    for stream in listener.incoming() {
        match stream.map(|s| handle_stream(s, oauth_client)) {
            Ok(tr) => return tr,
            Err(e) => handle_error(&e.into())?,
        }
    }

    Err(Error::NoIncomingStreams)
}

fn handle_stream(mut stream: TcpStream, oauth_client: &BasicClient) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
    let code: AuthorizationCode;
    // let state: CsrfToken;

    {
        let mut reader = BufReader::new(&mut stream);

        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;

        let redirect_url = request_line.split_whitespace().nth(1).unwrap();
        let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap(); // TODO: don't unwrap

        let code_pair = url
            .query_pairs()
            .find(|pair| pair.0 == "code")
            .unwrap(); // TODO: remove unwrap
        code = AuthorizationCode::new(code_pair.1.into_owned());

        /*let state_pair = url
            .query_pairs()
            .find(|pair| pair.0 == "state")
            .unwrap(); // TODO: remove unwrap
        state = CsrfToken::new(state_pair.1.into_owned());*/
    }

    let message = "Go back to your terminal :)";
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
        message.len(),
        message
    );
    stream.write_all(response.as_bytes())?;

    oauth_client.exchange_code(code).request(http_client)
        .map_err(std::convert::From::from)
}

/// Handles an error from [`listen`].
/// Returns an error if the error should cause [`listen`] to return that error.
#[allow(clippy::unnecessary_wraps)]
fn handle_error(err: &Error) -> Result<()> {
    println!("An error occurred: {err}");
    Ok(())
}

