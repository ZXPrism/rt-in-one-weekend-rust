# rt-in-one-weekend-rust

![demo.png](demo.png)

This is a rust implementation of the book series ["Ray Tracing in One Weekend"](https://raytracing.github.io/).

I made this mainly for learning Rust, since I believe that the best way to learn a language is to use it.

As for the reason of choosing to do a ray tracer...That's because, it is fun and romantic!



## Getting Started
- Install [rustup](https://rustup.rs/)
- Clone this repo, and run `cargo run --release`, wait for a while, then you will find the output `out.png` at the root folder.

## Core Logic
- `Camera` iterate on the 2d viewport surface's pixels and send rays (`Ray`) into the scene (`Scene`)
- for one pixel's each sample
    - `Scene` iterate on a list of objects (impl `Drawable`) and do ray-object intersection test
    - if hit, the object firstly fills in struct `HitInfo` **with basic information (normal + front_face flag)**
        - then call the `scatter` method of material (`Material`) bound to the object to calculate the scattered ray
            - in the method, the new scattered ray is calculated and the albedo is set in the `HitInfo`
    - `Scene` returns `HitInfo` to the `Camera`
        - based on max bounce settings etc., `Camera` continues to call hit tests of `Scene`
        - and finish the computation of one pixel

### Dataflow
Camera <-> Scene <-> Drawable <-> Material

## TODO
- [ ] Implement Book I
    - [X] Basics (pure on CPU)
    - [ ] Optimize using multi-threading
    - [ ] Optimize using compute shaders (using [wgpu](https://github.com/gfx-rs/wgpu))
- [ ] Implement Book II (TBD)
- [ ] Implement Book III (TBD)

## Confusion
- At commit `7b6a782ab9035127a860fb88cb3aa15cc1feca83`, file `camera.rs`, ln 95 (2025-07-30)
    - Why Lambertian reflection can be realized through adding a random vector on a sphere to the surface normal?

- At commit `28854e86ee7bc57f3cd34ba1027419818c5e7c76`, file `image_writer.rs`, ln 20 (2025-07-30)
    - Why the inverse of "gamma 2" is just a square root?
