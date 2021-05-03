use std::{error, fmt, io};

/// The most generic error message in ReQL
#[derive(Debug)]
pub enum Error {
    Compile(String),
    Runtime(Runtime),
    Client(Client),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Compile(msg) => write!(f, "compile error; {}", msg),
            Self::Runtime(msg) => write!(f, "runtime error; {}", msg),
            Self::Client(msg) => write!(f, "client error; {}", msg),
        }
    }
}

/// The parent class of all runtime errors
///
/// All errors on the server unrelated to compilation. Programs may use this to catch any runtime
/// error, but the server will always return a more specific error class.
#[derive(Debug)]
pub enum Runtime {
    /// The query contains a logical impossibility, such as adding a number to a string.
    QueryLogic(String),
    NonExistence(String),
    ResourceLimit(String),
    User(String),
    Internal(String),
    Availability(Availability),
    Permission(String),
}

impl From<Runtime> for Error {
    fn from(err: Runtime) -> Error {
        Error::Runtime(err)
    }
}

impl fmt::Display for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::QueryLogic(msg) => write!(f, "query logic; {}", msg),
            Self::NonExistence(msg) => write!(f, "non-existance error; {}", msg),
            Self::ResourceLimit(msg) => write!(f, "resource limit error; {}", msg),
            Self::User(msg) => write!(f, "user error; {}", msg),
            Self::Internal(msg) => write!(f, "internal error; {}", msg),
            Self::Availability(msg) => write!(f, "availability error; {}", msg),
            Self::Permission(msg) => write!(f, "permission error; {}", msg),
        }
    }
}

/// A server in the cluster is unavailable
///
/// The parent class of `OpFailedError` and `OpIndeterminateError`. Programs may use this
/// to catch any availability error, but the server will always return one of this class’s
/// children.
#[derive(Debug)]
pub enum Availability {
    OpFailed(String),
    OpIndeterminate(String),
}

impl From<Availability> for Error {
    fn from(err: Availability) -> Error {
        Runtime::Availability(err).into()
    }
}

impl fmt::Display for Availability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OpFailed(msg) => write!(f, "operation failed; {}", msg),
            Self::OpIndeterminate(msg) => write!(f, "operation indeterminate; {}", msg),
        }
    }
}

/// An error has occurred within the driver
///
/// This may be a driver bug, or it may be an unfulfillable command, such as an unserializable
/// query.
#[derive(Debug)]
#[non_exhaustive]
pub enum Client {
    Auth(String),
    ConnectionBroken,
    ConnectionLocked,
    Io(io::Error),
    Json(serde_json::Error),
    Other(String),
}

impl From<Client> for Error {
    fn from(err: Client) -> Error {
        Error::Client(err)
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Auth(msg) => write!(f, "auth error; {}", msg),
            Self::ConnectionBroken => write!(f, "connection broken"),
            Self::ConnectionLocked => write!(
                f,
                "another query is running a changefeed on this connection"
            ),
            Self::Io(error) => write!(f, "{}", error),
            Self::Json(error) => write!(f, "{}", error),
            Self::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Client::Io(err).into()
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Client::Json(err).into()
    }
}