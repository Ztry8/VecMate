# VecMate
## Lightweight, zero-dependency, type-agnostic library for vector math.   
```rust
let mut position = consts::f32::ZERO;
let target = vec2(10.0, 5.0);
let speed = 2.0;

let direction = (target - position).normalized();
position += direction * speed;

println!("Moving towards {target}");
println!("New position: {position}");
```

## Why?


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
