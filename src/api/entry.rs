use crate::err::RulesError;
use crate::orchestrator::Orchestrator;

pub fn evaluate() -> Result<(), RulesError> {
    Orchestrator::run()
}
