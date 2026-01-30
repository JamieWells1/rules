use rules_rs::{err::RulesError, parser::tags};

fn main() -> Result<(), RulesError> {
    let tags = tags::parse_tags()?;

    Ok(())
}
