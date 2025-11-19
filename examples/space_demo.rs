use taho_routes::{SpaceNetwork, Point3};

fn main() {
    let mut net = SpaceNetwork::new();
    let a = net.add_location(Point3::new(0.0, 0.0, 0.0));
    let b = net.add_location(Point3::new(1.0, 1.0, 0.0));
    let c = net.add_location(Point3::new(2.0, 0.0, 0.0));

    net.connect_bidirectional(a, b).unwrap();
    net.connect_bidirectional(b, c).unwrap();

    match net.shortest_route(a, c) {
        Some(route) => {
            let ids: Vec<_> = route.locations.iter().map(|id| id.index()).collect();
            println!("Route (by LocationId index): {:?}", ids);
            println!("Total distance: {}", route.total_distance);
        }
        None => {
            println!("No route found between {:?} and {:?}", a.index(), c.index());
        }
    }
}
