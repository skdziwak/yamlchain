#[derive(Debug)]
pub enum Error {
    InvalidEnvironment(String),
    OpenAIError(String),
    InterpolationError,
    VariableNotFound(String),
    RuntimeError(String),
    VariableTypeMismatch(String),
    MaxIterationsExceeded,
    StageError {
        stage_name: String,
        error: Box<Error>,
    },
}

impl Error {
    pub fn at_stage<S: Into<String>>(self, stage_name: S) -> Error {
        Error::StageError {
            stage_name: stage_name.into(),
            error: Box::new(self),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidEnvironment(env) => write!(f, "Invalid environment: {}", env),
            Error::OpenAIError(msg) => write!(f, "OpenAI error: {}", msg),
            Error::StageError { stage_name, error } => {
                write!(f, "Error in stage {}: {}", stage_name, error)
            },
            Error::InterpolationError => write!(f, "Interpolation error"),
            Error::VariableNotFound(var_name) => write!(f, "Variable not found: {}", var_name),
            Error::VariableTypeMismatch(msg) => write!(f, "Variable type mismatch: {}", msg),
            Error::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            Error::MaxIterationsExceeded => write!(f, "Max iterations exceeded"),
        }
    }
}
