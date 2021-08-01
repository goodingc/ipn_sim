use ipn_sim_lib::event::Event;
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::ipn_sim::ipn_sim_builder::IpnSimBuilder;
use ipn_sim_lib::report::Report;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct ReporterEvent;

impl Event for ReporterEvent {
    fn handle(self: Box<Self>, sim: &mut IpnSim) {}
}

struct TestReport(Shared<TestReportData>);

#[derive(Eq, PartialEq, Debug)]
struct TestReportData {
    init: bool,
    ticks: u64,
    end: bool,
}

impl Report for TestReport {
    fn on_init(&mut self, _sim: &IpnSim) {
        self.0.borrow_mut().init = true;
    }

    fn on_tick(&mut self, _sim: &IpnSim, _events: &Vec<Box<dyn Event>>) {
        self.0.borrow_mut().ticks += 1;
    }

    fn on_end(&mut self, _sim: &IpnSim) {
        self.0.borrow_mut().end = true;
    }
}

#[test]
fn test() {
    let mut sim_builder = IpnSimBuilder::new(1000);
    for i in 0..100 {
        sim_builder = sim_builder.add_event(i, ReporterEvent);
    }
    let data = Rc::new(RefCell::new(TestReportData {
        init: false,
        ticks: 0,
        end: false,
    }));

    let mut sim = sim_builder.add_report(TestReport(Rc::clone(&data))).build();

    sim.run();

    assert_eq!(
        *data.borrow(),
        TestReportData {
            init: true,
            ticks: 100,
            end: true
        }
    );
}
