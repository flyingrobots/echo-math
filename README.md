# echo-math

A deterministic math library for [Echo](https://github.com/flyingrobots/echo), a recursive metagraph (RMG) simulation engine.

This crate provides core, deterministic mathematical primitives essential for high-fidelity, reproducible simulations and applications. It is designed to be small, focused, and produce bit-for-bit identical results across supported platforms, making it ideal for:
*   **Deterministic Physics Simulations:** Ensuring consistent outcomes in complex physical interactions.
*   **Multiplayer Game Synchronization:** Maintaining perfect state synchronization across networked clients.
*   **Reproducible Scientific Computing:** Guaranteeing identical results for research and analysis.
*   **Financial Modeling:** Providing precision and determinism with fixed-point arithmetic.

## Features

`echo-math` is built on a robust, type-safe foundation to prevent common mathematical pitfalls at compile time:

*   **Comprehensive Scalar Trait Hierarchy:**
    *   `Scalar`: Basic arithmetic operations (`Add`, `Mul`, etc.).
    *   `RealScalar`: Adds "real" number operations like `sqrt`, `abs`, `min`, `max`.
    *   `TrigScalar`: Extends with trigonometric functions (`sin`, `cos`, `atan2`, etc.).
*   **Flexible Scalar Types:**
    *   `f32`: Standard non-deterministic floating-point for rendering and approximate physics.
    *   `F32Det`: A deterministic `f32` wrapper for controlled float behavior.
    *   `DFix64`: A 64-bit fixed-point type for absolute determinism and precision.
*   **Deterministic/Non-Deterministic Markers:** `DeterministicScalar` and `NondetScalar` traits enforce type correctness in sensitive contexts at compile time.
*   **Unit-Safe Angle Type:** `Angle<T, U>` (with `RadAngle` and `DegAngle` aliases) prevents accidental mixing of radians and degrees, centralizing "float-dangerous" angle operations.
*   **Generic Vector Types:** `Vec3<T>` provides a unified API for 3D vectors, generic over any `Scalar` type.
*   **Semantic Newtypes:** `Point3<T>` and `Direction3<T>` enforce physical correctness, preventing nonsensical operations (e.g., adding two points) at compile time.
*   **Compile-Time Safety Enforcement:** Utilizes `trybuild` tests to guarantee that the type system's invariants are upheld, making it impossible to introduce common math footguns.

## Installation

Add `echo-math` to your `Cargo.toml` dependencies:

```toml
[dependencies]
echo-math = "0.1.0" # Or the latest version
```

## Usage

```rust
use echo_math::{
    Scalar, RealScalar, TrigScalar,
    DeterministicScalar, NondetScalar,
    F32Det, DFix64,
    Vec3, Point3, Direction3,
    RadAngle, DegAngle,
};

// Example: Deterministic simulation step
fn step_deterministic_entity<T: DeterministicScalar>(
    position: &mut Point3<T>,
    velocity: Direction3<T>,
    dt: T,
) {
    *position = *position + (velocity * dt);
}

// Example: Non-deterministic rendering
fn render_object<T: NondetScalar>(
    camera_pos: Point3<T>,
    fov: DegAngle<T>,
) {
    // ... use fov.to_radians().sin() for calculations
}

fn main() {
    // Using f32 (non-deterministic)
    let pos_f32 = Point3::<f32>::new(0.0, 0.0, 0.0);
    let dir_f32 = Direction3::<f32>::new(1.0, 0.0, 0.0);
    let angle_f32 = DegAngle::<f32>::from_degrees(45.0);

    // Using F32Det (deterministic float)
    let mut pos_det = Point3::<F32Det>::new(F32Det(0.0), F32Det(0.0), F32Det(0.0));
    let vel_det = Direction3::<F32Det>::new(F32Det(1.0), F32Det(0.0), F32Det(0.0));
    let dt_det = F32Det(0.016);
    step_deterministic_entity(&mut pos_det, vel_det, dt_det);

    // Using DFix64 (fixed-point)
    let pos_fixed = Point3::<DFix64>::new(DFix64::from_f32(0.0), DFix64::from_f32(0.0), DFix64::from_f32(0.0));
    // ... fixed-point operations
}