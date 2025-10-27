# VecMate
[![GitHub last commit](https://img.shields.io/github/last-commit/ztry8/vecmate)](https://github.com/ztry8/vecmate/commits)
[![crates.io](https://img.shields.io/crates/v/vecmate)](https://crates.io/crates/vecmate)
[![docs.rs](https://img.shields.io/docsrs/vecmate)](https://docs.rs/vecmate)
[![License](https://img.shields.io/github/license/ztry8/vecmate)](https://github.com/ztry8/vecmate/blob/main/LICENSE)
## Lightweight, zero-dependency, type-agnostic library for vector math.   
```rust
let mut position = consts::f32::ZERO;
let target = vec2(10.0, 5.0);
let speed = 2.0;

let direction = (target - position).normalize();
position += direction * speed;

println!("Moving towards {target}");
println!("New position: {position}");
```

## Why?
I didn't find any simple libraries for vector mathematics, so I decided to write my own!   
VecMate is focused on minimalism and clear API

## Contributing
Any pull-requests and issues are welcome!   
Please remember that one of the goals of the library is to be dependency-free,   
Therefore, any proposals that contain dependencies will be rejected.   
Please adhere to the architecture and style of the project.

## TODO
- [ ] More constants
- [ ] More functions
- [ ] 3D vectors
- [ ] 4D vectors
