#[derive(Debug)]
pub enum Error {
    ClientCreation(reqwest::Error),
    CommunicationError(reqwest::Error),
    InvalidResponse(reqwest::StatusCode),
    JsonParse(reqwest::Error),
}
