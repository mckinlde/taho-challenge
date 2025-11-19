// happypath.rs
// “Typical usage” / smoke tests that mirror what a consumer would do.

use taho_routes::{SpaceNetwork, Point3};

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn simple_direct_route() {
    let mut net = SpaceNetwork::new();
    let a = net.add_location(Point3::new(0.0, 0.0, 0.0));
    let b = net.add_location(Point3::new(3.0, 4.0, 0.0));

    net.connect_bidirectional(a, b).unwrap();

    let route = net.shortest_route(a, b).expect("route");
    assert_eq!(route.locations, vec![a, b]);
    assert!(approx_eq(route.total_distance, 5.0));
}

#[test]
fn typical_three_hop_path() {
    let mut net = SpaceNetwork::new();
    let earth = net.add_location(Point3::new(0.0, 0.0, 0.0));
    let luna  = net.add_location(Point3::new(1.0, 0.0, 0.0));
    let mars  = net.add_location(Point3::new(2.0, 0.0, 0.0));

    net.connect_bidirectional(earth, luna).unwrap();
    net.connect_bidirectional(luna, mars).unwrap();

    let route = net.shortest_route(earth, mars).expect("route");

    assert_eq!(route.locations, vec![earth, luna, mars]);
    assert!(route.total_distance > 0.0);
}
