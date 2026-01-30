use rules_rs::orchestrator::Orchestrator;
use rules_rs::err::RulesError;

fn main() -> Result<(), RulesError> {
    Orchestrator::run()
}
