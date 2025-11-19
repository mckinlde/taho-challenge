use taho_routes::{SpaceNetwork, Point3};

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn triangle_routes_via_middle_when_direct_edge_missing() {
    let mut net = SpaceNetwork::new();
    let a = net.add_location(Point3::new(0.0, 0.0, 0.0));
    let b = net.add_location(Point3::new(2.0, 0.0, 0.0));
    let c = net.add_location(Point3::new(1.0, 1.0, 0.0));

    // Only connect via C
    net.connect_bidirectional(a, c).unwrap();
    net.connect_bidirectional(c, b).unwrap();
    // no direct a <-> b edge

    let route = net.shortest_route(a, b).expect("route should exist");

    assert_eq!(route.locations, vec![a, c, b]);

    // Check the distance is what we expect:
    let d_ac = net.location(a).unwrap().position.distance_to(&net.location(c).unwrap().position);
    let d_cb = net.location(c).unwrap().position.distance_to(&net.location(b).unwrap().position);
    assert!(approx_eq(route.total_distance, d_ac + d_cb));
}

#[test]
fn directed_line_is_one_way_only() {
    let mut net = SpaceNetwork::new();
    let a = net.add_location(Point3::new(0.0, 0.0, 0.0));
    let b = net.add_location(Point3::new(1.0, 0.0, 0.0));
    let c = net.add_location(Point3::new(2.0, 0.0, 0.0));

    // One-way line: A -> B -> C
    net.connect_directed(a, b).unwrap();
    net.connect_directed(b, c).unwrap();

    // Forward direction works
    let forward = net.shortest_route(a, c);
    assert!(forward.is_some());
    let forward_route = forward.unwrap();
    assert_eq!(forward_route.locations, vec![a, b, c]);

    // Backwards should fail: no path from C back to A
    assert!(net.shortest_route(c, a).is_none());
}