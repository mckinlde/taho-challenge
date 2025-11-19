// shapes.rs
// Graph “geometry” and algorithm behavior: triangles, squares, directed behavior, etc.

use taho_routes::{SpaceNetwork, Point3};

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn triangle_prefers_short_leg() {
    let mut net = SpaceNetwork::new();
    let a = net.add_location(Point3::new(0.0, 0.0, 0.0));
    let b = net.add_location(Point3::new(2.0, 0.0, 0.0));
    let c = net.add_location(Point3::new(1.0, 0.0, 0.0));

    // Long edge
    net.connect_bidirectional(a, b).unwrap(); // length 2.0

    // Two short edges via C
    net.connect_bidirectional(a, c).unwrap(); // 1.0
    net.connect_bidirectional(c, b).unwrap(); // 1.0

    let route = net.shortest_route(a, b).unwrap();

    assert_eq!(route.locations, vec![a, c, b]);
    assert!(approx_eq(route.total_distance, 2.0));
}

#[test]
fn directed_square_one_way_only() {
    let mut net = SpaceNetwork::new();
    let a = net.add_location(Point3::new(0.0, 0.0, 0.0));
    let b = net.add_location(Point3::new(1.0, 0.0, 0.0));
    let c = net.add_location(Point3::new(1.0, 1.0, 0.0));
    let d = net.add_location(Point3::new(0.0, 1.0, 0.0));

    // Directed cycle A -> B -> C -> D -> A
    net.connect_directed(a, b).unwrap();
    net.connect_directed(b, c).unwrap();
    net.connect_directed(c, d).unwrap();
    net.connect_directed(d, a).unwrap();

    // Route exists in the “forward” direction
    assert!(net.shortest_route(a, c).is_some());
    // But not backwards
    assert!(net.shortest_route(c, a).is_none());
}
