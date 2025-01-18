# RustRay
_A simple Rust-based ray tracer_

## About

This is, first and foremost, a project with which I am learning the ins-and-outs of Rust's features. I seriously doubt I'll be able to get decent performance out of it, but I'll be benchmarking it nonetheless

## Goals

- Implement a basic suite of primitives:
  - Plane (TODO)
  - Sphere (IN PROGRESS)
  - Cube (TODO)
  - Triangle (stretch)
- Implement lighting
  - Ambient (TODO)
  - Diffuse (TODO)
  - Specular (TODO)
- Implement surface effects
  - Diffusion (TODO)
  - Reflectivity (TODO)
  - Transparency (stretch) 
- Benchmark scenes of increasing complexity each time a "release" is completd.
- Explore NVidia-specific technologies like CUDA to enhance performance (stretch)

## Methodology
I'll be benchmarking the generated binary with a consistent set of scenes and hardware.

Hardware used:
- CPU: AMD R7 7800X3D
- RAM: 32GB DDR5
- GPU: NVIDIA RTX 4090
- OS: Windows 10

I'll also be tagging "versions" as I implement, or revise, a component of the tracer.


## Acknowledgements
Whenever I use a specific online resource to derive a math formula or optimization trick, I'll leave a link directly in the comments of the relevent code. Please take a read. For all other, more general references, please see below.
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Advanced Graphics, University of Cambridge](https://www.cl.cam.ac.uk/teaching/1718/AdvGraph/1.%20Ray%20Tracing%20-%20All%20the%20Maths.pdf)