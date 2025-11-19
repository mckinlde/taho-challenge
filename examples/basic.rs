use taho_routes::{SpaceNetwork, Point3};

fn main() {
    let mut net = SpaceNetwork::new();

    let earth = net.add_location(Point3::new(0.0, 0.0, 0.0));
    let mars  = net.add_location(Point3::new(1.0, 0.0, 0.0));
    let luna  = net.add_location(Point3::new(0.0, 0.5, 0.0));

    // Earth <-> Luna <-> Mars
    net.connect_bidirectional(earth, luna).unwrap();
    net.connect_bidirectional(luna, mars).unwrap();

    let route = net.shortest_route(earth, mars).expect("route exists");
    println!("route length: {}", route.total_distance);
    println!("hops: {}", route.len());
}
