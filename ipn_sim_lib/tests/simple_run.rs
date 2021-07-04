use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::ipn_sim::ipn_sim_builder::IpnSimBuilder;
use ipn_sim_lib::movements::static_movement::StaticMovement;
use ipn_sim_lib::cgmath::{EuclideanSpace, Point3};
use ipn_sim_lib::routers::test_router::TestRouter;
use ipn_sim_lib::movements::path_movement::PathMovement;
use ipn_sim_lib::events::create_message_event::CreateMessageEvent;
use std::rc::Rc;
use ipn_sim_lib::utils::Data;
use ipn_sim_lib::transceiver::transceiver::Transceiver;
use ipn_sim_lib::transceiver::transceive_guards::simple_transceive_guard::SimpleTransceiveGuard;

#[test]
pub fn simple_scenario() {
    let mut sim = IpnSimBuilder::new(1_000_000_000 * 3600 * 24)
        .add_node(
            "static-node",
            StaticMovement::new(Point3::origin()),
            TestRouter,
            Transceiver::new(1., SimpleTransceiveGuard::new(1.496e+11))
        ).add_node(
        "path-node",
        PathMovement::new(vec![
            (0, Point3::new(0., 0., 0.)),
            (1_000_000_000 * 3600 * 6, Point3::new(0., 0., 1.496e+11)),
            (1_000_000_000 * 3600 * 12, Point3::new(1.496e+11, 0., 1.496e+11)),
            (1_000_000_000 * 3600 * 18, Point3::new(1.496e+11, 0., 0.)),
            (1_000_000_000 * 3600 * 24, Point3::new(0., 0., 0.)),
        ]),
        TestRouter,
        Transceiver::new(1., SimpleTransceiveGuard::new(1.496e+11))
    ).build();

    sim.add_event(10, CreateMessageEvent {
        node: sim.get_node(0),
        data: Data::from_bytes("test".as_bytes()),
    });

    sim.init();
    sim.run();
}