use rules::orchestrator::Orchestrator;
use rules::err::RulesError;

fn main() -> Result<(), RulesError> {
    Orchestrator::run()
}
