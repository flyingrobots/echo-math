# echo-math

A deterministic math library for the Echo engine ecosystem.

This crate provides core, deterministic mathematical primitives required by the Echo engine and related projects. It is designed to be small, focused, and produce bit-for-bit identical results across supported platforms.

## Features

*   **Vec3**: A 3D vector type.
*   **Quat**: A quaternion type for rotations.
*   **Mat4**: A 4x4 matrix type for transformations.
*   **Prng**: A deterministic pseudo-random number generator.
*   **Scalar**: A trait for abstracting over deterministic scalar types (e.g., fixed-point vs. floating-point).

## License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for details.
