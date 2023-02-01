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
///             "0 devision!!!".to_string()
///         ))
///     }
/// 
///     Ok(a/b)
/// }
/// 
/// let result = devide(6, 3).expect("devision error");
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
        write!(f, "{} kind error: \"{}\"", self.error.debug(), self.reason)
    }
}
impl std::fmt::Debug for CursedErrorHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(self.error.debug())
            .field(&self.reason)
            .finish()
    }
}
impl std::error::Error for CursedErrorHandle {}

/// enum with kinds of errors
pub enum CursedError {
    InvalidArgument,
    InvalidCommand,
    InvalidOption,
    Initialize,
    ThreadJoin,
    NotEnought,
    TimeOut,
    Sockets,
    TooMany,
    Parse,
    OS
}

impl CursedError {
    pub fn debug(&self) -> &'static str {
        match *self {
            CursedError::InvalidArgument => "invalid argument",
            CursedError::InvalidCommand => "invalid command",
            CursedError::InvalidOption => "invalid option",
            CursedError::Initialize => "initialize",
            CursedError::ThreadJoin => "thread join",
            CursedError::NotEnought => "not enought",
            CursedError::TimeOut => "time out",
            CursedError::Sockets => "sockets",
            CursedError::TooMany => "too many",
            CursedError::Parse => "parse",
            CursedError::OS => "OS",
        }
    }
}