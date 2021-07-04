

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TickResult {
    NoMoreEvents,
    MoreEvents,
    SimulationEnd,
}

impl TickResult {
    pub fn is_terminal(&self) -> bool {
        match self {
            TickResult::NoMoreEvents => true,
            TickResult::MoreEvents => false,
            TickResult::SimulationEnd => true,
        }
    }
}