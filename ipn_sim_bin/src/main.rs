mod format_time;
mod router_log_report;

use crate::router_log_report::RouterLogReport;
use ipn_sim_lib::cgmath::{EuclideanSpace, Point3};
use ipn_sim_lib::events::create_message_event::{CreateMessageEvent, MessageDestination};
use ipn_sim_lib::ipn_sim::ipn_sim_builder::IpnSimBuilder;
use ipn_sim_lib::movements::orbital_movement::OrbitalMovement;
use ipn_sim_lib::movements::static_movement::StaticMovement;
use ipn_sim_lib::routers::epidemic::epidemic::Epidemic;
use ipn_sim_lib::routers::epidemic::flavours::ack::Ack;
use ipn_sim_lib::transceiver::transceive_guards::simple::SimpleTransceiveGuard;
use ipn_sim_lib::transceiver::transceiver::Transceiver;
use ipn_sim_lib::utils::{Data, NodeId, SpaceMetric, TimeMetric};
use std::f64::consts::PI;
use std::fs::{File, OpenOptions};

fn main() {
    orbiting_rings()
        // .add_report(RouterLogReport(
        //     OpenOptions::new()
        //         .write(true)
        //         .open("C:\\Users\\callu\\Documents\\Programming\\brand_new_ipn_sim\\ipn_sim_bin\\out\\out.txt").unwrap()
        // ))
        .build()
        .run();
}

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

    let radius_step = max_ring_radius / ring_sizes.len() as f64;

    for (ring_index, ring_size) in ring_sizes.iter().enumerate() {
        let offset_step = 2. * PI / *ring_size as f64;
        for index in 0..*ring_size {
            builder = builder.add_node(
                format!("node-{}-{}", ring_index, index),
                OrbitalMovement::new(
                    &earth,
                    radius_step * (ring_index + 1) as f64,
                    0.,
                    offset_step * index as f64,
                    0.,
                    ring_index % 2 == 0,
                ),
                Epidemic::<Ack>::new(1024 * 8, 1_000_000_000 * 3600),
                Transceiver::new(
                    1.,
                    SimpleTransceiveGuard::new(
                        2.2 * max_ring_radius / ring_sizes.len() as SpaceMetric,
                    ),
                ),
                1024,
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
                destination: MessageDestination::Single(
                    rand::random::<NodeId>() % node_count as NodeId,
                ),
                // destination: MessageDestination::All,
                payload: "Hello there, World!".into(),
                ttl: Some(time + 1_000_000_000 * 3600 * 12),
            },
        )
    }

    builder
}
