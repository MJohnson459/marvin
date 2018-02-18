use std::error::Error;

pub trait MarvinError: Send + 'static {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&(MarvinError)> {
        None
    }
}

impl MarvinError for Box<MarvinError> {
    fn description(&self) -> &str {
        (**self).description()
    }
    fn cause(&self) -> Option<&MarvinError> {
        (**self).cause()
    }
}

impl<T: MarvinError> MarvinError for Box<T> {
    fn description(&self) -> &str {
        (**self).description()
    }
    fn cause(&self) -> Option<&MarvinError> {
        (**self).cause()
    }
}

pub type MarvinResult<T> = Result<T, Box<MarvinError>>;

struct ConcreteMarvinError {
    description: String,
    detail: Option<String>,
    cause: Option<Box<MarvinError>>,
}

impl MarvinError for ConcreteMarvinError {
    fn description(&self) -> &str {
        &self.description
    }
    fn cause(&self) -> Option<&MarvinError> {
        self.cause.as_ref().map(|c| &**c)
    }
}
