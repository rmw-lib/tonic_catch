pub use tonic_catch_proc::tonic_catch;

use tonic::{Response, Status};

pub type Result<T> = std::result::Result<Response<T>, Status>;

pub struct Error(anyhow::Error);

impl<T:Into<anyhow::Error>> From<T> for Error {
  fn from(err: T) -> Error {
    Error(err.into())
  }
}

impl From<Error> for Status {
  fn from(err: Error) -> Status {
    Status::new(tonic::Code::Internal, err.0.to_string())
  }
}

