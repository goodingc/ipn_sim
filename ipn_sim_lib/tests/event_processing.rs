use ipn_sim_lib::event::Event;
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::ipn_sim::ipn_sim_builder::IpnSimBuilder;
use ipn_sim_lib::ipn_sim::tick_result::TickResult;
use std::cell::RefCell;
use std::rc::Rc;
use ipn_sim_lib::utils::shared;

#[test]
fn test_empty_schedule() {
    let mut sim = IpnSimBuilder::new(0).build();
    assert_eq!(sim.run().0, TickResult::NoMoreEvents);
    assert_eq!(sim.time, 0);
}

#[derive(Clone)]
struct TestEvent;

impl Event for TestEvent {
    fn handle(self: Box<Self>, sim: &mut IpnSim) {}
}

#[test]
fn test_event_times() {
    let mut sim = IpnSimBuilder::new(100).build();
    for i in (0..100).step_by(10) {
        sim.add_event(i, TestEvent);
    }
    for i in (0..100).step_by(10) {
        assert_eq!(sim.tick(), TickResult::MoreEvents);
        assert_eq!(sim.time, i);
    }
}

#[derive(Clone)]
struct IncrementerEvent(Shared<usize>);

impl Event for IncrementerEvent {
    fn handle(self: Box<Self>, sim: &mut IpnSim) {
        *self.0.borrow_mut() += 1;
    }
}

#[test]
fn test_events_handled() {
    let mut sim = IpnSimBuilder::new(100).build();
    let counter = shared(0);

    for i in (0..100).step_by(10) {
        sim.add_event(i, IncrementerEvent(Rc::clone(&counter)));
    }

    assert_eq!(sim.run().0, TickResult::NoMoreEvents);
    assert_eq!(sim.time, 90);

    assert_eq!(*counter.borrow(), 10);
}
