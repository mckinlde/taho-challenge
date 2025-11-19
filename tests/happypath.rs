use taho_routes::{SpaceNetwork, Point3};

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn trivial_same_start_and_goal() {
        let mut net = SpaceNetwork::new();
        let a = net.add_location(Point3::new(0.0, 0.0, 0.0));

        let route = net.shortest_route(a, a).expect("route should exist");
        assert_eq!(route.locations, vec![a]);
        assert!(approx_eq(route.total_distance, 0.0));
    }

    #[test]
    fn direct_edge_route() {
        let mut net = SpaceNetwork::new();
        let a = net.add_location(Point3::new(0.0, 0.0, 0.0));
        let b = net.add_location(Point3::new(3.0, 4.0, 0.0)); // distance 5

        net.connect_bidirectional(a, b).unwrap();

        let route = net.shortest_route(a, b).expect("route should exist");
        assert_eq!(route.locations, vec![a, b]);
        assert!(approx_eq(route.total_distance, 5.0));
    }

    #[test]
    fn picks_shorter_path_over_longer_one() {
        let mut net = SpaceNetwork::new();
        let a = net.add_location(Point3::new(0.0, 0.0, 0.0));
        let b = net.add_location(Point3::new(9.0, 0.0, 0.0)); // long-ish
        let c = net.add_location(Point3::new(3.0, 0.0, 0.0));
        let d = net.add_location(Point3::new(6.0, 0.0, 0.0));

        // direct edge
        net.connect_bidirectional(a, b).unwrap(); // distance 9

        // multi-hop but shorter
        net.connect_bidirectional(a, c).unwrap(); // 3
        net.connect_bidirectional(c, d).unwrap(); // 3
        net.connect_bidirectional(d, b).unwrap(); // 3
        // total = 9 as well; tweak positions to make the multi-hop path < 9
        net.move_location(b, Point3::new(10.0, 0.0, 0.0)).unwrap(); // direct = 10

        let route = net.shortest_route(a, b).expect("route should exist");
        assert_eq!(route.locations, vec![a, c, d, b]);
        assert!(approx_eq(route.total_distance, 9.0));
    }

    #[test]
    fn disconnected_nodes_have_no_route() {
        let mut net = SpaceNetwork::new();
        let a = net.add_location(Point3::new(0.0, 0.0, 0.0));
        let b = net.add_location(Point3::new(1.0, 0.0, 0.0));
        let c = net.add_location(Point3::new(2.0, 0.0, 0.0));

        net.connect_bidirectional(a, b).unwrap();
        // c is isolated

        assert!(net.shortest_route(a, c).is_none());
    }

    #[test]
    fn moving_a_location_changes_route_distance() {
        let mut net = SpaceNetwork::new();
        let a = net.add_location(Point3::new(0.0, 0.0, 0.0));
        let b = net.add_location(Point3::new(1.0, 0.0, 0.0));

        net.connect_bidirectional(a, b).unwrap();

        let route1 = net.shortest_route(a, b).unwrap();
        assert!(approx_eq(route1.total_distance, 1.0));

        net.move_location(b, Point3::new(2.0, 0.0, 0.0)).unwrap();

        let route2 = net.shortest_route(a, b).unwrap();
        assert!(approx_eq(route2.total_distance, 2.0));
    }
}
