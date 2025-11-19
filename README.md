## `README.md` for walkthrough

# taho_routes

`taho_routes` is a small, focused Rust crate for computing shortest routes
between locations in space, connected by straight-line paths.

The mental model is a set of waypoints (supply stations in space, hyperspace
gates, etc.) with connections between them. A *route* is an ordered sequence
of connected locations; the *shortest* route minimizes total distance.

This crate is designed as if it were headed toward production use at TAHO:
clear API, simple data structures, and easy recomputation when the network
changes.

---

## Core concepts

### Coordinates

- Locations live in **3D Euclidean space**, represented by:

  ```rust
  pub struct Point3 {
      pub x: f64,
      pub y: f64,
      pub z: f64,
  }
  ```

* Distance is straight-line:

  ```rust
  impl Point3 {
      pub fn distance_to(&self, other: &Self) -> f64 { /* sqrt(dx²+dy²+dz²) */ }
  }
  ```

The coordinate system is intentionally simple; if TAHO later wants a different
metric (2D, lat/long, game coordinates), the `Point3`/`distance_to` abstraction
is the only place that needs to change.

### Location identity

Locations are referenced by an opaque handle:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocationId(usize);
```

* Callers treat `LocationId` as a handle, not a raw index.
* `LocationId::index()` is exposed only for debugging / logging.
* Internally, locations are stored in `Vec<Location>` for cache-friendly
  access and simple mutation.

### Graph model

The graph is represented by `SpaceNetwork`:

```rust
pub struct SpaceNetwork {
    locations: Vec<Location>,
    adjacency: Vec<Vec<LocationId>>,
}
```

* `locations[i]` holds the `Location` (id + position).
* `adjacency[i]` is the list of neighbors for location `i`.
* Edges themselves have **no attributes**; edge weight is derived solely
  from the coordinates of the endpoints.

Supported operations:

* `SpaceNetwork::new()`
* `add_location(position: Point3) -> LocationId`
* `move_location(id: LocationId, new_position: Point3)`
  (changes distances from that node)
* `connect_bidirectional(a, b)`
  (undirected edge, e.g. normal hyperspace lane)
* `connect_directed(from, to)`
  (directed edge, for one-way lanes)
* `neighbors(id)` for read-only graph inspection

### Routes

A `Route` bundles the sequence of locations and the total distance:

```rust
pub struct Route {
    pub locations: Vec<LocationId>,
    pub total_distance: f64,
}
```

Creation is internal; callers get `Route` from:

```rust
SpaceNetwork::shortest_route(start: LocationId, goal: LocationId) -> Option<Route>
```

---

## Shortest path algorithm

Routing uses **Dijkstra’s algorithm** over the current snapshot of the network.

Key details:

* Priority queue: `BinaryHeap<State>`
* Each `State` holds:

  * `cost: f64` (wrapped in `OrderedFloat` so it can be ordered)
  * `position: LocationId`
* Edge weights are computed on the fly via `Point3::distance_to`.
* The algorithm:

  * Initializes `dist[start] = 0`, all others `∞`.
  * Pops the lowest-cost node from the heap.
  * Relaxes edges to neighbors.
  * Stops early once `goal` is popped.
  * Reconstructs the path using a `prev: Vec<Option<LocationId>>`.

Result:

* `Some(Route)` if a path exists.
* `None` if the graph is disconnected between `start` and `goal`.

Because distances are derived from coordinates only, any change to positions
or connectivity is automatically reflected in subsequent `shortest_route` calls.

---

## Example usage

Simple example (also available as `examples/space_demo.rs`):

```rust
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
```

Running locally:

```bash
# Run tests
cargo test

# Run the example
cargo run --example space_demo
```

---

## Testing strategy

Tests live in two places:

1. **Unit tests** in `src/lib.rs`
   Focused on small behaviors:

   * Straight-line two-node path
   * Multi-hop routing
   * Disconnected components

2. **Integration tests** in `tests/`
   These exercise the public API as a consumer would:

   * `tests/happypath.rs`
     “Normal” usage: build a small network, compute a route, check distance.

   * `tests/shapes.rs`
     Graph-shape behavior:

     * Triangle where the only path is via an intermediate node.
     * Directed line (`A -> B -> C`) that is one-way only.

   * `tests/edgecase.rs`
     Edge cases:

     * start == goal
     * moving a location changes route distance
     * no route between components, etc.

The emphasis is on **clarity over cleverness**: all tests are small, readable,
and use fixed coordinates so they’re easy to reason about in a code review.

---

## Design choices & tradeoffs

### Why `Vec` + `LocationId`?

* Simple, cache-friendly, and easy to mutate.
* `LocationId` is just a thin wrapper over `usize`, so it’s zero-cost in practice.
* No external dependencies needed for graph storage.

If the network ever needs dynamic deletion or more complex semantics, it’s still
possible to evolve this design (e.g., by introducing tombstones or stable handles).

### Why Dijkstra (not A*, etc.)?

* The problem statement doesn’t require heuristics or approximate routing.
* Dijkstra is:

  * Easy to implement correctly.
  * Easy to reason about in interviews / code reviews.
  * Sufficient for the likely graph sizes in this exercise.

If later we need large-scale routing or bounding box search, it’s straightforward
to add A* on top, reusing the same graph representation.

### No external crates

Per the prompt’s suggestion:

* Graph structure, priority queue, and math all use the Rust standard library.
* This keeps the crate small, portable, and easy to understand.

If this were a larger production system, it might make sense to use:

* `petgraph` (for graph algorithms), or
* `ordered-float` (for robust floating-point ordering, NaN handling, etc.).

---

## How to extend

Some obvious next steps that would integrate cleanly with the current design:

* **Alternative metrics**: swap out Euclidean distance for:

  * 2D coordinates, or
  * weighted distances (e.g. travel cost, fuel, risk).
* **Edge attributes**:

  * Introduce a separate edge storage to hold capacities, costs, etc.
* **Route constraints**:

  * Avoid specific waypoints or regions.
  * Limit maximum hops or distance.
* **Serialization**:

  * Add support for saving/loading networks from disk (e.g. via serde).

---

## Notes on AI assistance

This crate was implemented with help from an AI assistant as allowed by the
challenge instructions. Design notes are checked in under:

* `chatGPT_Design.md` — design sketch and notes

During a walkthrough, I can elaborate on:

* The reasoning behind data structures (`Vec` + `LocationId`, adjacency lists).
* Dijkstra’s algorithm implementation details.
* Tradeoffs between clarity and performance, and where I’d optimize or extend
  for a real production deployment at TAHO.
