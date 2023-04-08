//! Utility crate for differentiating fatal and non fatal errors
use std::error::Error as StdError;

/// An error that can never happend
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NeverErr {}

impl std::fmt::Display for NeverErr {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match *self {} }
}

impl std::error::Error for NeverErr {}

/// Error type
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FatalError<E> {
    /// Error
    Error(E),
    /// Fatal error
    Fatal(E),
}

impl<E> FatalError<E> {
    /// return true if this error non is fatal
    pub fn is_error(&self) -> bool { matches!(self, FatalError::Error(_)) }

    /// return true if this error is fatal
    pub fn is_fatal(&self) -> bool { matches!(self, FatalError::Fatal(_)) }

    /// transforms the error into it's inner type
    pub fn into_inner(self) -> E {
        match self {
            FatalError::Error(x) => x,
            FatalError::Fatal(x) => x,
        }
    }

    /// applies f to the inner error preserving the [`FatalError::Error`] or [`FatalError::Fatal`] state
    pub fn map<E2, F>(self, f: F) -> FatalError<E2>
    where
        F: FnOnce(E) -> E2,
    {
        match self {
            FatalError::Error(x) => FatalError::Error(f(x)),
            FatalError::Fatal(x) => FatalError::Fatal(f(x)),
        }
    }

    /// Makes this error fatal
    pub fn escalate(self) -> Self { FatalError::Fatal(self.into_inner()) }

    /// Makes this error non fatal
    pub fn deescalate(self) -> Self { FatalError::Error(self.into_inner()) }

    /// Return `Ok(E)` if the error is non fatal else `Err(Self)` is returned
    pub fn fatality(self) -> Result<E, Self> {
        match self {
            FatalError::Error(x) => Ok(x),
            x @ FatalError::Fatal(_) => Err(x),
        }
    }

    /// return `Err(E)` if the error is fatal otherwise `Ok(E)` is returned
    pub fn recover(self) -> Result<E, E> {
        match self {
            FatalError::Error(x) => Ok(x),
            FatalError::Fatal(x) => Err(x),
        }
    }

    /// recover a non fatal error with the given closure
    pub fn map_error<T, F>(self, f: F) -> Result<T, Self>
    where
        F: FnOnce(E) -> Result<T, Self>,
    {
        match self {
            FatalError::Error(x) => f(x),
            x @ FatalError::Fatal(_) => Err(x),
        }
    }

    /// recover from a fatal error with the given closure
    pub fn map_fatal<T, F>(self, f: F) -> Result<T, Self>
    where
        F: FnOnce(E) -> Result<T, Self>,
    {
        match self {
            x @ FatalError::Error(_) => Err(x),
            FatalError::Fatal(x) => f(x),
        }
    }

    /// recover from either an error or a fatal error with the given closure
    pub fn then<T, F>(self, f: F) -> Result<T, Self>
    where
        F: FnOnce(E) -> Result<T, Self>,
    {
        f(self.into_inner())
    }
}

impl<E: std::fmt::Display> std::fmt::Display for FatalError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FatalError::Error(x) => write!(f, "Error: {x}"),
            FatalError::Fatal(x) => write!(f, "Fatal Error: {x}"),
        }
    }
}

impl<E: StdError + 'static> StdError for FatalError<E> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            FatalError::Error(x) | FatalError::Fatal(x) => x.source(),
        }
    }
}
