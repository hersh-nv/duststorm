# duststorm

Experiments with visualisers, written in nannou for Rust

## Usage

### Perlin 1

```cargo run --bin perlin1```

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
-  Hue wheel

### Perlin 2

```cargo run --bin perlin2```

A cloud of agents pushing through 3-dimensional Perlin noise

R to reset

Spacebar to re-seed the Perlin noise

### Voronoi bubbles

```cargo run --bin voronoi```

A mesh of bubbles pushing against each other until they move to equilibrium

Left mouse button to add bubble

Right mouse button to pop bubble

R to reset
