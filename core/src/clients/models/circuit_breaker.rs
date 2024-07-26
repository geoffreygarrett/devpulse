use failsafe::{FailurePolicy, Instrument, StateMachine};

struct ClientBreaker<POLICY: FailurePolicy + Send + Sync, INSTRUMENT: Instrument + Send + Sync> {
    circuit_breaker: StateMachine<POLICY, INSTRUMENT>,
}
