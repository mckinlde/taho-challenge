// shortestroute.rs
// Edge cases and invariants around the routing itself:

use taho_routes::{SpaceNetwork, Point3};

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn start_equals_goal_zero_distance() {
    let mut net = SpaceNetwork::new();
    let a = net.add_location(Point3::new(42.0, -7.0, 3.14));

    let route = net.shortest_route(a, a).expect("route");
    assert_eq!(route.locations, vec![a]);
    assert!(approx_eq(route.total_distance, 0.0));
}

#[test]
fn disconnected_components_have_no_route() {
    let mut net = SpaceNetwork::new();
    let a = net.add_location(Point3::new(0.0, 0.0, 0.0));
    let b = net.add_location(Point3::new(1.0, 0.0, 0.0));
    let c = net.add_location(Point3::new(10.0, 0.0, 0.0));

    net.connect_bidirectional(a, b).unwrap();
    // c is isolated

    assert!(net.shortest_route(a, c).is_none());
}

#[test]
fn moving_location_updates_route_distance() {
    let mut net = SpaceNetwork::new();
    let a = net.add_location(Point3::new(0.0, 0.0, 0.0));
    let b = net.add_location(Point3::new(1.0, 0.0, 0.0));

    net.connect_bidirectional(a, b).unwrap();

    let original = net.shortest_route(a, b).unwrap();
    assert!(approx_eq(original.total_distance, 1.0));

    net.move_location(b, Point3::new(3.0, 4.0, 0.0)).unwrap();

    let updated = net.shortest_route(a, b).unwrap();
    assert!(approx_eq(updated.total_distance, 5.0));
}
