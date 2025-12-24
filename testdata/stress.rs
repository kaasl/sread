// Stress test file for sread - Rust
// Contains functions, structs, enums, traits, impl blocks, generics, etc.

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::sync::Arc;

// Simple function
fn simple_function() -> i32 {
    42
}

// Function with generics
fn identity<T>(value: T) -> T {
    value
}

// Function with complex signature
fn complex_signature<T, U>(
    first: T,
    second: U,
    optional: Option<String>,
) -> Result<(T, U), &'static str>
where
    T: Clone + Display,
    U: Default,
{
    if optional.is_some() {
        Ok((first, second))
    } else {
        Err("missing optional")
    }
}

// Async function
async fn fetch_data(url: &str) -> Result<String, String> {
    Ok(format!("data from {}", url))
}

// Simple struct
struct SimpleStruct {
    value: i32,
}

// Struct with generics
struct GenericStruct<T, U> {
    first: T,
    second: U,
    cache: HashMap<String, T>,
}

// Tuple struct
struct Point(f64, f64, f64);

// Unit struct
struct Marker;

// Struct with lifetimes
struct BorrowedData<'a> {
    data: &'a str,
    length: usize,
}

// Simple enum
enum Status {
    Pending,
    Active,
    Completed,
    Failed(String),
}

// Enum with data
enum Message {
    Text(String),
    Number(i64),
    Data { id: u32, payload: Vec<u8> },
    Empty,
}

// Enum with generics
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Simple trait
trait Processor {
    fn process(&self, input: &str) -> String;
    fn validate(&self) -> bool {
        true
    }
}

// Trait with generics
trait Repository<T> {
    fn find_by_id(&self, id: u64) -> Option<T>;
    fn save(&mut self, entity: T) -> Result<(), String>;
    fn delete(&mut self, id: u64) -> bool;
}

// Trait with associated types
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// Complex struct for impl blocks
struct ComplexService {
    name: String,
    config: HashMap<String, String>,
    counter: u64,
}

// Impl block
impl ComplexService {
    // Constructor
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            config: HashMap::new(),
            counter: 0,
        }
    }

    // Method with &self
    fn get_name(&self) -> &str {
        &self.name
    }

    // Method with &mut self
    fn increment(&mut self) -> u64 {
        self.counter += 1;
        self.counter
    }

    // Method with generics
    fn transform<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&str) -> R,
    {
        f(&self.name)
    }

    // Async method
    async fn async_operation(&self) -> String {
        format!("async result for {}", self.name)
    }
}

// Trait impl
impl Processor for ComplexService {
    fn process(&self, input: &str) -> String {
        format!("{}: {}", self.name, input)
    }

    fn validate(&self) -> bool {
        !self.name.is_empty()
    }
}

// Display impl
impl Display for ComplexService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ComplexService({})", self.name)
    }
}

// Generic impl
impl<T, U> GenericStruct<T, U>
where
    T: Clone,
{
    fn get_first(&self) -> T {
        self.first.clone()
    }
}

// Module
mod inner {
    pub fn inner_function() -> &'static str {
        "from inner module"
    }

    pub struct InnerStruct {
        pub value: i32,
    }
}

// Const and static
const MAX_SIZE: usize = 1024;
static GLOBAL_NAME: &str = "sread";

// Type alias
type StringMap = HashMap<String, String>;

// Function with closures
fn with_closure<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(10)
}

// Function returning impl trait
fn make_iterator() -> impl Iterator<Item = i32> {
    vec![1, 2, 3].into_iter()
}

// Unsafe function
unsafe fn dangerous_operation(ptr: *const i32) -> i32 {
    *ptr
}

// Macro-like function (not a macro, just named like one)
fn create_handler() -> Box<dyn Fn() -> String> {
    Box::new(|| "handler".to_string())
}

// Function at end
fn final_function() -> &'static str {
    "I'm at the end!"
}
