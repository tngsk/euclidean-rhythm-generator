# Euclidean Rhythm Generator

A Rust implementation for generating musical rhythm patterns using Bjorklund's algorithm, distributing a number of pulses as evenly as possible across a number of steps.

## Usage

```rust
let rhythm = euclidean_rhythm(3, 8); // Cuban tresillo
println!("E(3, 8) = [{}]", rhythm_to_string(&rhythm)); // [x..x..x.]
```

### Core Functions
- `euclidean_rhythm(pulses, steps)`: Generates a Euclidean rhythm.
- `rotate_rhythm(rhythm, rotation)`: Rotates a rhythm pattern.

### Examples ('x' = beat, '.' = rest)
- E(3,8): Cuban tresillo `[x..x..x.]`
- E(5,8): Cuban cinquillo `[x.xx.xx.]`
- E(5,16): Bossa-nova `[x..x..x..x..x...]`

## License
MIT