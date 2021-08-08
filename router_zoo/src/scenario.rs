use std::f64::consts::PI;

use ipn_sim_lib::cgmath::{EuclideanSpace, Point3};
use ipn_sim_lib::events::create_message_event::{CreateMessageEvent, MessageDestination};
use ipn_sim_lib::ipn_sim::ipn_sim_builder::IpnSimBuilder;
use ipn_sim_lib::movements::orbital_movement::OrbitalMovement;
use ipn_sim_lib::movements::static_movement::StaticMovement;
use ipn_sim_lib::node::node_builder::NodeBuilder;
use ipn_sim_lib::router::Router;
use ipn_sim_lib::routers::epidemic::epidemic::Epidemic;
use ipn_sim_lib::routers::epidemic::flavours::ack::Ack;
use ipn_sim_lib::transceiver::transceive_guards::simple::SimpleTransceiveGuard;
use ipn_sim_lib::transceiver::transceiver::Transceiver;
use ipn_sim_lib::utils::{NodeId, SpaceMetric, TimeMetric};

pub struct Scenario {
    pub name: String,
    pub builder: Box<dyn Fn(Box<dyn Router>) -> IpnSimBuilder>,
}

impl Scenario {
    pub fn new(name: impl ToString, builder: impl Fn(Box<dyn Router>) -> IpnSimBuilder + 'static) -> Self {
        Self {
            name: name.to_string(),
            builder: Box::new(builder),
        }
    }

    pub fn build(&self, router: Box<dyn Router>) -> IpnSimBuilder {
        (self.builder)(router)
    }

    pub fn scenarios() -> Vec<Scenario> {
        vec![
            Scenario::new(
                "Orbiting Rings",
                |router| {
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
                                NodeBuilder::new()
                                    .name(format!("node-{}-{}", ring_index, index))
                                    .movement(OrbitalMovement::new(
                                        &earth,
                                        radius_step * (ring_index + 1) as f64,
                                        0.,
                                        offset_step * index as f64,
                                        0.,
                                        ring_index % 2 == 0,
                                    )).message_buffer_size(1024)
                                    .boxed_router(router.clone())
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
                },
            ),
            Scenario::new(
                "Constellation",
                |router| {
                    let plane_count = 10;
                    let plane_size = 10;
                    let plane_radius = 550_000. + 6.371e6;
                    let plane_inclination = (53. / 360.) * 2. * PI;

                    let mut builder = IpnSimBuilder::new(1_000_000_000 * 3600 * 24).add_body(
                        "Earth",
                        5.972e24,
                        StaticMovement::new(Point3::origin()),
                        6.371e6,
                    );

                    let earth = builder.get_body(0);

                    let ascending_node_offset_step = (2. * PI) / plane_count as f64;
                    let offset_step = (2. * PI) / plane_size as f64;

                    for plane_index in 0..plane_count {
                        for node_index in 0..plane_size {
                            builder = builder.add_node(
                                NodeBuilder::new()
                                    .name(format!("node-{}-{}", plane_index, node_index))
                                    .movement(OrbitalMovement::new(
                                        &earth,
                                        plane_radius,
                                        plane_inclination,
                                        offset_step * node_index as f64,
                                        ascending_node_offset_step * plane_index as f64,
                                        true,
                                    )).message_buffer_size(1024)
                                    .boxed_router(router.clone())
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
                                payload: "Hello there, World!".into(),
                                ttl: Some(time + 1_000_000_000 * 3600 * 12),
                            },
                        )
                    }

                    builder
                },
            ),
            Scenario::new(
                "Grid",
                |router| {
                    let node_x_count = 20;
                    let node_z_count = 20;
                    let node_separation = 5_000_000 as SpaceMetric;

                    let x_offset_distance = (node_x_count as SpaceMetric / 2. - 0.5) * node_separation;
                    let z_offset_distance = (node_z_count as SpaceMetric / 2. - 0.5) * node_separation;

                    let mut builder = IpnSimBuilder::new(1_000_000_000 * 3600 * 24);

                    for node_x_index in 0..node_x_count {
                        for node_z_index in 0..node_z_count {
                            builder = builder.add_node(
                                NodeBuilder::new()
                                    .name(format!("{}-{}", node_x_index, node_z_index))
                                    .movement(StaticMovement::new(Point3::new(
                                        node_x_index as SpaceMetric * node_separation - x_offset_distance,
                                        0.,
                                        node_z_index as SpaceMetric * node_separation - z_offset_distance,
                                    ))).message_buffer_size(1024)
                                    .boxed_router(router.clone())
                                    .transceive_speed(1.)
                                    .transceive_guard(SimpleTransceiveGuard::new(
                                        node_separation
                                    )),
                            )
                        }
                    }

                    let node_count = node_x_count * node_z_count;

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
                },
            ),
        ]
    }
}