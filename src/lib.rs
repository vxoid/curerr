/// struct created for error handling
/// 
/// # Examples
/// ```
/// use cursederror::*;
/// 
/// fn devide(a: i32, b: i32) -> Result<i32, CursedErrorHandle> {
///     if b == 0 {
///         return Err(CursedErrorHandle::new(
///             CursedError::InvalidArgument,
///             "0 division!!!".to_string()
///         ))
///     }
/// 
///     Ok(a/b)
/// }
/// 
/// let result = devide(6, 3).expect("division error");
/// 
/// assert_eq!(result, 2)
/// ```
pub struct CursedErrorHandle {
    error: CursedError,
    reason: String,
}

impl CursedErrorHandle {
    pub fn new(error: CursedError, reason: String) -> Self {
        Self { error, reason }
    }
    pub fn get_error(&self) -> &CursedError {
        &self.error
    }
    pub fn get_reason(&self) -> &String {
        &self.reason
    }
}

impl std::fmt::Display for CursedErrorHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} kind of error: \"{}\"", self.error.to_str(), self.reason)
    }
}
impl std::fmt::Debug for CursedErrorHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(self.error.to_str())
            .field(&self.reason)
            .finish()
    }
}
impl std::error::Error for CursedErrorHandle {}

/// enum with kinds of errors
pub enum CursedError {
    InsufficientBuffer,
    NotEnoughMemory,
    InvalidArgument,
    NotImplemented,
    AlreadyExists,
    AccessDenied,
    Initialize,
    ThreadJoin,
    NotEnough,
    NotFound,
    Invalid,
    Aborted,
    Timeout,
    TooMany,
    NoError,
    Parse,
    Write,
    Read
}

impl CursedError {
    pub fn to_str(&self) -> &'static str {
        match self {
            CursedError::InsufficientBuffer => "insufficient buffer",
            CursedError::NotEnoughMemory => "not enough memory",
            CursedError::InvalidArgument => "invalid argument",
            CursedError::NotImplemented => "not implemented",
            CursedError::AlreadyExists => "already exists",
            CursedError::AccessDenied => "access denied",
            CursedError::Initialize => "initialize",
            CursedError::ThreadJoin => "thread join",
            CursedError::NotEnough => "not enought",
            CursedError::NotFound => "not found",
            CursedError::Timeout => "time out",
            CursedError::TooMany => "too many",
            CursedError::NoError => "no error",
            CursedError::Aborted => "aborted",
            CursedError::Invalid => "invalid",
            CursedError::Parse => "parse",
            CursedError::Write => "write",
            CursedError::Read => "read",
        }
    }
}

impl ToString for CursedError {
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}