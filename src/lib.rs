pub mod args;
pub mod error;
pub mod response;

use crate::args::Args;
use crate::error::Error;
use reqwest::blocking::ClientBuilder;
use crate::response::Response;

pub fn query_wiki(args: Args) -> Result<Response, Error> {
    let client = ClientBuilder::new()
        .build()
        .map_err(Error::ClientCreation)?;

    let response = client
        .get(&format!(
            "https://{}.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json",
            args.locale,
            args.search
        ))
        .send()
        .map_err(Error::CommunicationError)?;

    let status = response.status();

    if !status.is_success() {
        return Err(Error::InvalidResponse(status))
    }

    response.json().map_err(Error::JsonParse)
}
