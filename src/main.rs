mod error;
use error::RuleEngineError;

fn main() {
    println!("Hello, world!");
    let a: i32 = 5;
    if a < 4 {
        let err = RuleEngineError::FileNotFound("No file found!".to_string());
        println!("Error {:?}", err);
    }
}
