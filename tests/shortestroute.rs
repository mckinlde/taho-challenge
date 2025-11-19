use taho_routes::{SpaceNetwork, Point3};

#[test]
fn typical_usage_build_graph_and_route() {
    let mut net = SpaceNetwork::new();

    let earth = net.add_location(Point3::new(0.0, 0.0, 0.0));
    let luna  = net.add_location(Point3::new(1.0, 0.0, 0.0));
    let mars  = net.add_location(Point3::new(2.0, 0.0, 0.0));

    net.connect_bidirectional(earth, luna).unwrap();
    net.connect_bidirectional(luna, mars).unwrap();

    let route = net.shortest_route(earth, mars).unwrap();

    // This is essentially your "README example" in test form.
    assert_eq!(route.locations.len(), 3);
    assert_eq!(route.locations[0], earth);
    assert_eq!(route.locations[2], mars);
}
