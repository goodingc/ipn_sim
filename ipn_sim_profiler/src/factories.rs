use std::f64::consts::PI;
use std::fs::{File, OpenOptions};

use ipn_sim_lib::cgmath::{EuclideanSpace, Point3};
use ipn_sim_lib::events::create_message_event::CreateMessageEvent;
use ipn_sim_lib::ipn_sim::ipn_sim_builder::IpnSimBuilder;
use ipn_sim_lib::message_destination::MessageDestination;
use ipn_sim_lib::movements::orbital_movement::OrbitalMovement;
use ipn_sim_lib::movements::static_movement::StaticMovement;
use ipn_sim_lib::node::node_builder::NodeBuilder;
use ipn_sim_lib::routers::{epidemic_2, epidemic_2::epidemic::Ack};
use ipn_sim_lib::routers::epidemic::epidemic::Epidemic;
// use ipn_sim_lib::routers::epidemic::flavours::ack::Ack;
use ipn_sim_lib::transceiver::transceive_guards::simple::SimpleTransceiveGuard;
use ipn_sim_lib::transceiver::transceiver::Transceiver;
use ipn_sim_lib::utils::{Data, NodeId, SpaceMetric, TimeMetric};

pub fn orbiting_rings() -> IpnSimBuilder {
    let ring_sizes = [6, 6, 6, 6, 6, 6];
    let max_ring_radius = 42164e3 * 2.;
    let sim_length = 1_000_000_000 * 3600 * 24;

    let mut builder = IpnSimBuilder::new(sim_length).add_body(
        "Earth",
        5.972e24,
        StaticMovement::new(Point3::origin()),
        6.371e6,
    );

    let earth = builder.get_body(0);

    let radius_step = max_ring_radius / ring_sizes.len() as SpaceMetric;

    for (ring_index, ring_size) in ring_sizes.iter().enumerate() {
        let offset_step = 2. * (PI as SpaceMetric) / *ring_size as SpaceMetric;
        for index in 0..*ring_size {
            builder = builder.add_node(
                NodeBuilder::new()
                    .name(format!("node-{}-{}", ring_index, index))
                    .movement(OrbitalMovement::new(
                        &earth,
                        radius_step * (ring_index + 1) as SpaceMetric,
                        0.,
                        offset_step * index as SpaceMetric,
                        0.,
                        ring_index % 2 == 0,
                    )).message_buffer_size(1024)
                    // .router(Epidemic::<Ack>::new(1024 * 8, 1_000_000_000 * 3600))
                    .router(epidemic_2::epidemic::Epidemic::new(
                        1024 * 8,
                        1_000_000_000 * 60 * 15,
                        false,
                        Ack::Bilateral,
                        false
                    ))
                    .transceive_speed(1.)
                    .transceive_guard(SimpleTransceiveGuard::new(
                        2.2 * max_ring_radius / ring_sizes.len() as SpaceMetric,
                    ))
            )
        }
    }

    let node_count = ring_sizes.iter().sum::<u32>();

    for i in 0..400 {
        let node = builder.get_node(rand::random::<usize>() % node_count as usize);
        let time = rand::random::<TimeMetric>() % 1_000_000_000 * 3600 * 6;
        builder = builder.add_event(
            time,
            // rand::random::<TimeMetric>() % sim_length,
            CreateMessageEvent {
                node,
                destination: MessageDestination::<NodeId>::Single(
                    rand::random::<NodeId>() % node_count as NodeId,
                ),
                // destination: MessageDestination::All,
                payload: "Hello there, World!".as_bytes().to_vec().into_boxed_slice(),
                ttl: Some(time + 1_000_000_000 * 3600 * 12),
            },
        )
    }

    builder
}

pub fn constellation() -> IpnSimBuilder {
    let plane_count = 10;
    let plane_size = 10;
    let plane_radius = 550_000. + 6.371e6;
    let plane_inclination = (53. / 360.) * 2. * PI as SpaceMetric;

    let mut builder = IpnSimBuilder::new(1_000_000_000 * 3600 * 24).add_body(
        "Earth",
        5.972e24,
        StaticMovement::new(Point3::origin()),
        6.371e6,
    );

    let earth = builder.get_body(0);

    let ascending_node_offset_step = (2. * PI as SpaceMetric) / plane_count as SpaceMetric;
    let offset_step = (2. * PI as SpaceMetric) / plane_size as SpaceMetric;

    for plane_index in 0..plane_count {
        for node_index in 0..plane_size {
            builder = builder.add_node(
                NodeBuilder::new()
                    .name(format!("node-{}-{}", plane_index, node_index))
                    .movement(OrbitalMovement::new(
                        &earth,
                        plane_radius,
                        plane_inclination,
                        offset_step * node_index as SpaceMetric,
                        ascending_node_offset_step * plane_index as SpaceMetric,
                        true,
                    )).message_buffer_size(1024)
                    // .router(Epidemic::<Ack>::new(1024 * 8, 1_000_000_000 * 3600))
                    .router(epidemic_2::epidemic::Epidemic::new(
                        1024 * 8,
                        1_000_000_000 * 60 * 15,
                        false,
                        Ack::Bilateral,
                        false
                    ))
                    .transceive_speed(1.)
                    .transceive_guard(SimpleTransceiveGuard::new(
                        0.5 * plane_radius as SpaceMetric,
                    ))
            );
        }
    }

    let node_count = plane_count * plane_size;

    for i in 0..400 {
        let node = builder.get_node(rand::random::<usize>() % node_count as usize);
        let time = rand::random::<TimeMetric>() % 1_000_000_000 * 3600 * 6;
        builder = builder.add_event(
            time,
            // rand::random::<TimeMetric>() % sim_length,
            CreateMessageEvent {
                node,
                destination: MessageDestination::Single(
                    rand::random::<NodeId>() % node_count as NodeId,
                ),
                // destination: MessageDestination::All,
                payload: "Hello there, World!".as_bytes().to_vec().into_boxed_slice(),
                ttl: Some(time + 1_000_000_000 * 3600 * 12),
            },
        )
    }

    builder
}