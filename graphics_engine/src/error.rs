use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("could not get an adapter")]
    AdapterRequestError,
}
