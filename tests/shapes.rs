use taho_routes::{SpaceNetwork, Point3};

#[test]
fn directed_edges_respected() {
    let mut net = SpaceNetwork::new();
    let a = net.add_location(Point3::new(0.0, 0.0, 0.0));
    let b = net.add_location(Point3::new(1.0, 0.0, 0.0));

    // only a -> b
    net.connect_directed(a, b).unwrap();

    assert!(net.shortest_route(a, b).is_some());
    assert!(net.shortest_route(b, a).is_none());
}
