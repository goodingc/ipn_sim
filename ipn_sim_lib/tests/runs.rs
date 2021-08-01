
use ipn_sim_lib::ipn_sim::ipn_sim_builder::IpnSimBuilder;
use ipn_sim_lib::movements::static_movement::StaticMovement;
use ipn_sim_lib::cgmath::{EuclideanSpace, Point3};
use ipn_sim_lib::routers::test_router::TestRouter;
use ipn_sim_lib::movements::path_movement::PathMovement;
use ipn_sim_lib::events::create_message_event::{CreateMessageEvent, MessageDestination};

use ipn_sim_lib::utils::Data;
use ipn_sim_lib::transceiver::transceiver::Transceiver;
use ipn_sim_lib::transceiver::transceive_guards::simple::SimpleTransceiveGuard;
use ipn_sim_lib::movements::orbital_movement::OrbitalMovement;
use std::f64::consts::PI;
use ipn_sim_lib::routers::epidemic::epidemic_router::EpidemicRouter;

#[test]
pub fn simple_scenario() {
    let mut sim = IpnSimBuilder::new(1_000_000_000 * 3600 * 24)
        .add_node(
            "static-node",
            StaticMovement::new(Point3::origin()),
            TestRouter,
            Transceiver::new(1., SimpleTransceiveGuard::new(1.496e+11))
                1024
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
        Transceiver::new(1., SimpleTransceiveGuard::new(1.496e+11)),
        1024
    ).build();

    sim.add_event(10, CreateMessageEvent {
        node: sim.get_node(0),
        destination: MessageDestination::All,
        payload: "test".into(),
    });

    sim.run();
}

#[test]
fn orbiting_rings() {
    let ring_sizes = [4, 4, 4, 4, 4];
    let max_ring_radius = 42164e3;

    let mut builder = IpnSimBuilder::new(1_000_000_000 * 3600 * 24)
        .add_body("Earth", 5.972e24, StaticMovement::new(Point3::origin()), 6.371e6);

    let earth = builder.get_body(0);

    let radius_step = max_ring_radius / ring_sizes.len() as f64;

    for (ring_index, ring_size) in ring_sizes.iter().enumerate() {
        let offset_step = 2. * PI / *ring_size as f64;
        for index in 0..*ring_size {
            builder = builder
                .add_node(
                    format!("node-{}-{}", ring_index, index),
                    OrbitalMovement::new(
                        &earth,
                        radius_step * (ring_index + 1) as f64,
                        0.,
                        offset_step * index as f64,
                        0.,
                        ring_index % 2 == 0
                    ),
                    EpidemicRouter::new(64, 1_000_000_000 * 3600),
                    Transceiver::new(1., SimpleTransceiveGuard::new(1.496e+11)),
                    1024
                )
        }
    }

    let node = builder.get_node(0);
    let mut sim = builder.add_event(10, CreateMessageEvent {
        node,
        destination: MessageDestination::All,
        payload: "test".into(),
    }).build();

    sim.run();
}