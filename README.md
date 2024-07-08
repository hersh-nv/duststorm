# duststorm

Experiments with visualisers, written in nannou for Rust

## Usage

### Perlin

```cargo run --bin perlin```

A cloud of agents pushing through 3-dimensional Perlin noise

R to reset agents

Spacebar to re-seed the noise field

D to cycle targetting modes:
-  Circle
-  Figure eight
-  Floating: target point tracks the current average of the agent positions
-  Cursor: target point is the mouse cursor

C to cycle colour modes:
-  White
-  Red/blue

### Perlin2

```cargo run --bin perlin2```

A different take on the same Perlin logic, without a target attractor point

R to reset agents

Spacebar to re-seed the noise field

### Voronoi

```cargo run --bin voronoi```

A mesh of bubbles pushing against each other until they move to equilibrium

Left mouse button to add bubble

Right mouse button to pop bubble
