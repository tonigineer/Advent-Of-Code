use std::fmt::{self, Display};

/// Represents the answer to a problem, which can be a string, integer, float, or unimplemented.
///
/// The `Answer` enum is used to encapsulate different types of answers that might be produced
/// by solving a problem or a puzzle. It provides variants for strings, integers, floating-point numbers,
/// and an `Unimplemented` variant for cases where the solution is not yet provided.
///
/// # Variants
///
/// - `String(String)`: Represents an answer that is a string.
/// - `Number(u64)`: Represents an answer that is an integer number.
/// - `Float(f64)`: Represents an answer that is a floating-point number.
/// - `Unimplemented`: Represents a placeholder for an unimplemented answer.
///
/// # Example
///
/// ```rust
/// use common::Answer;
///
/// let ans_str = Answer::String("Hello".to_string());
/// let ans_num = Answer::Number(42);
/// let ans_float = Answer::Float(3.14);
/// let ans_unimplemented = Answer::Unimplemented;
/// ```
#[derive(Debug, PartialEq)]
pub enum Answer {
    String(String),
    Number(u64),
    Float(f64),
    Unimplemented,
}

/// Implements the `Display` trait for `Answer`, allowing it to be formatted as a string.
///
/// This enables instances of `Answer` to be printed using the `{}` formatter, which will
/// display the contained value appropriately based on its variant.
///
/// # Example
///
/// ```rust
/// use common::Answer;
///
/// let ans = Answer::Number(42);
/// println!("{}", ans); // Outputs: 42
/// ```
impl Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Answer::String(s) => write!(f, "{s}"),
            Answer::Number(n) => write!(f, "{n}"),
            Answer::Float(n) => write!(f, "{n}"),
            Answer::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}

/// Converts a `String` into an `Answer::String` variant.
///
/// This allows a `String` to be seamlessly converted into an `Answer`, facilitating
/// easy wrapping of string results.
///
/// # Example
///
/// ```rust
/// use common::Answer;
///
/// let ans: Answer = String::from("Result").into();
/// ```
impl From<String> for Answer {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

/// Converts a string slice (`&str`) into an `Answer::String` variant.
///
/// This implementation allows string slices to be directly converted into `Answer`,
/// making it convenient to use string literals as answers.
///
/// # Example
///
/// ```rust
/// use common::Answer;
///
/// let ans: Answer = "Result".into();
/// ```
impl From<&str> for Answer {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

/// Macro to implement the `From` trait for `Answer` for multiple numeric types.
///
/// This macro generates `From` trait implementations for the `Answer` enum, converting
/// various numeric types into either `Answer::Number` or `Answer::Float` variants.
///
/// # Parameters
///
/// - `$variant`: The `Answer` variant (`Number` or `Float`) to map the types to.
/// - `$variant_type`: The internal type used by the variant (`u64` for `Number`, `f64` for `Float`).
/// - `{ $($type:ty),* $(,)? }`: A list of numeric types to implement `From` for.
/// ```
macro_rules! answer_impl {
    ($answer:ident, $answer_type:ty, { $($type:ty),* }) => {
        $(impl From<$type> for Answer {
            fn from(n: $type) -> Self {
                Self::$answer(n as $answer_type)
            }
        })*
    };
}

// Implement `From` for various integer types, mapping them to `Answer::Number`.
answer_impl!(
    Number, u64,
    { u8, u16, u32, u64, usize, i8, i16, i32, i64, isize }
);

// Implement `From` for floating-point types, mapping them to `Answer::Float`.
answer_impl!(
    Float, f64,
    { f32, f64 }
);
