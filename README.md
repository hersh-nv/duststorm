# duststorm

Experiments with visualisers, written in nannou for Rust

## Usage

### Perlin

```cargo run --bin perlin```

A cloud of agents pushing through 3-dimensional Perlin noise

R to reset

D to cycle targetting modes:
-  Circle
-  Figure eight
-  Floating: target point tracks the current average of the agent positions
-  Cursor: target point is the mouse cursor

C to cycle colour modes:
-  White
-  Red/blue

### Voronoi

```cargo run --bin voronoi```

Voronoi cells drawn around a cloud of wandering agents
