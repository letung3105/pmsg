mod chunk_type;

// TODO: custom error types
pub type Error = Box<dyn std::error::Error>;

pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    todo!()
}
