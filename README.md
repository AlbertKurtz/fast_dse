# Rust-Python Crystal Scattering Library

This project exposes high-performance Rust functions to Python using PyO3 and maturin. It provides simple utilities to generate 3D atomic point lattices and compute a Debye scattering-like intensity using parallel Rust code. A small Python script demonstrates how to call the Rust functions and plot the results.

- Rust module name (as seen from Python): `build_crystal`
- Python demo entry point: `main.py`

## Contents

- `src/lib.rs` — Rust implementation of the Python module using PyO3 and Rayon
- `main.py` — Example Python script that calls the Rust functions and plots results
- `pyproject.toml` — maturin/PyO3 build configuration
- `Cargo.toml` — Rust crate configuration

## What the library provides (`src/lib.rs`)

The Rust library defines a Python module named `build_crystal` with two functions:

1) `crystal(shape: str, lattice_param: float, length: float) -> list[list[float]]`
   - Generates 3D lattice points with a simple step equal to `lattice_param`.
   - Supported `shape` values:
     - `"cube"`: all lattice points in a cube of side `length`.
     - `"sphere"`: lattice points inside a sphere of diameter `length` (radius `length/2`).
   - Returns: a list of 3D points `[x, y, z]` (floats) representing atom positions.

2) `dse_optimized(min_q: float, max_q: float, q_step: float, crystal: list[list[float]]) -> list[float]`
   - Computes an intensity profile over `q` in `[min_q, max_q)` with spacing `q_step`.
   - Internally:
     - Precomputes all pairwise squared distances between points in `crystal`.
     - For each `q`, sums `sin(q * r) / (q * r)` over all pairs, with `r=0` contributing `1.0`.
     - Uses Rayon for parallelism over `q` values.
   - Returns: a list of intensities with length `floor((max_q - min_q) / q_step)`.

These functions are exported to Python with PyO3 via the `#[pymodule]` named `build_crystal`.

## Demo script (`main.py`)

`main.py` shows how to:
- Import the Rust functions from Python: `from build_crystal import crystal, dse_optimized`
- Generate two crystals (sphere and cube)
- Compute intensities for a `q` range
- Plot both curves using matplotlib

Snippet:
```python
from build_crystal import crystal, dse_optimized
import matplotlib.pyplot as plt
import numpy as np

lattice_param = 3.89
length = 30
sphere_crystal = crystal("sphere", lattice_param, length)
cube_crystal = crystal("cube", lattice_param, length)

q_0, q_f, q_step = 1.0, 15.0, 0.1
q_array = np.arange(q_0, q_f, q_step)
intensity_sphere = dse_optimized(q_0, q_f, q_step, sphere_crystal)
intensity_cube = dse_optimized(q_0, q_f, q_step, cube_crystal)

plt.plot(q_array, intensity_sphere, label="sphere")
plt.plot(q_array, intensity_cube, label="cube")
plt.legend()
plt.xlabel("q")
plt.ylabel("Intensity")
plt.title("Debye-like Scattering Intensity")
plt.show()
```

## Prerequisites

- Rust toolchain (stable) — install via https://rustup.rs
- Python 3.8+ (matching your environment)
- maturin — `pip install maturin`
- NumPy and Matplotlib for the demo — `pip install numpy matplotlib`

On Windows, ensure that your Python architecture (e.g., 64-bit) matches the Rust toolchain target. Using a recent MSVC build chain via Visual Studio Build Tools is recommended.

## Build and develop with maturin

This project is configured for maturin with PyO3.

Option A — develop in-place (editable install):

```bash
# from the project root
maturin develop
```

This compiles the Rust extension and installs it into your current Python environment so you can immediately `import build_crystal`.

Option B — build a wheel:

```bash
maturin build
# find the .whl in target/wheels/ and install it
pip install target\wheels\<your_wheel_name>.whl  # Windows
# or
pip install target/wheels/<your_wheel_name>.whl   # macOS/Linux
```

Note: If you manage Python with virtual environments, activate the env before running `maturin`.

## Running the demo

After building with `maturin develop` and installing Python dependencies:

```bash
python main.py
```

This will open a plot window comparing the intensity for a spherical and cubic crystal with the parameters from the script.

## Performance notes

- `dse_optimized` uses Rayon to parallelize over `q` values and precomputes all pairwise distances for cache efficiency.
- For large crystals, the distance matrix is O(N^2) in memory and time to precompute; adjust `length` and `lattice_param` accordingly.

## Troubleshooting

- ImportError: No module named `build_crystal`
  - Make sure you ran `maturin develop` (or installed the built wheel) in the same Python environment you use to run `main.py`.
- Linker/build errors on Windows
  - Ensure you have the MSVC toolchain and that Rust targets your Python architecture.
- Mismatched Python environment
  - `python -c "import sys; print(sys.executable)"` to confirm which interpreter you are using.

## License

MIT
