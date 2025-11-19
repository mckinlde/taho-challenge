### High-level design

- Coordinate system: 3D Euclidean Point3 { x, y, z } with distance_to.

- Identity: LocationId(usize) is a stable handle into internal vectors (no external crates).

- Graph: SpaceNetwork owns:

    - locations: Vec<Location>

    - adjacency: Vec<Vec<LocationId>> for connections (undirected by default).

- Routing: SpaceNetwork::shortest_route(start, goal) uses Dijkstra on the current network snapshot.

    - Edge weights are computed on the fly from coordinates (straight-line).

    - Returns Option<Route> where Route holds ordered LocationIds and total_distance.

- Mutation: you can add locations, connect them, and move locations (changing coordinates, thus changing distances) and then call shortest_route again.

No external crates, just std (including BinaryHeap).

### How this supports “recalculate when network changes”

- Adding/moving locations via add_location / move_location updates the underlying graph.

- Changing connectivity via connect_bidirectional / connect_directed updates adjacency lists.

- Each call to shortest_route runs Dijkstra on the current state of the network, so you can:

    - Modify the network arbitrarily.

    - Re-call shortest_route for updated optimal routes.

If you want to show AI usage in the repo, you can literally drop your prompt + planning notes into e.g. notes/ai_assist.md alongside this crate.