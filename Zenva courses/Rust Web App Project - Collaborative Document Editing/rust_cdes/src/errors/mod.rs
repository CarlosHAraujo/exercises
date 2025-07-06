use std::fmt::{Formatter, Display};

// Represents errors that can occur when creating a WebSocket server
pub enum ConnectionError {
  CreateServerError(String), // Describes an error that occurs when creating the WebSocket server
}

impl Display for ConnectionError {
  // Formats the connection error for display as a human-readable string
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match *self {
      ConnectionError::CreateServerError(ref desc) => {
        write!(f, "Error while creating WebSocket server: {}", desc)
      }
    }
  }
}
