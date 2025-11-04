# fast_dse: Fast Debye Scattering Equation

This project exposes high-performance Rust functions to Python using PyO3 and maturin. It provides **simplified** utilities to generate 3D atomic point lattices (simple cubic only) and compute a Debye scattering-like intensity using parallel Rust code. This is an educational/demonstration project showing Rust-Python integration for scientific computing.

- Rust module name (as seen from Python): `fast_dse`
- Python demo entry point: `main.py`

## Contents

- `src/lib.rs` — Rust implementation of the Python module using PyO3 and Rayon
- `main.py` — Example Python script that calls the Rust functions and plots results
- `pyproject.toml` — maturin/PyO3 build configuration
- `Cargo.toml` — Rust crate configuration
- `LICENSE` — MIT License

## What the library provides (`src/lib.rs`)

The Rust library defines a Python module named `fast_dse` with two functions:

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

These functions are exported to Python with PyO3 via the `#[pymodule]` named `fast_dse`.

## Important Limitations

This is a **simplified implementation** intended for educational purposes and basic simulations:

- **Crystal structures**: The `crystal()` function generates simple cubic lattices only. It does **not** support common crystal structures like FCC (face-centered cubic), BCC (body-centered cubic), HCP (hexagonal close-packed), or other Bravais lattices.

- **Atomic scattering factors**: The `dse_optimized()` function does **not** account for atomic scattering factors (form factors). This implementation is only accurate (up to a coefficient) for **monoatomic crystals** with identical scattering atoms. For real materials with different atom types or accurate X-ray/neutron scattering calculations, proper form factors must be included.

For production use in materials science or crystallography, consider using established libraries like [pymatgen](https://pymatgen.org/), [ASE](https://wiki.fysik.dtu.dk/ase/), or [diffpy](https://www.diffpy.org/).

## Demo script (`main.py`)

`main.py` shows how to:
- Import the Rust functions from Python: `from fast_dse import crystal, dse_optimized`
- Generate two crystals (sphere and cube)
- Compute intensities for a `q` range
- Plot both curves using matplotlib

Snippet:
```python
from fast_dse import crystal, dse_optimized
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

## Installation

**Prerequisites:**
- Rust toolchain (stable) — install via https://rustup.rs
- Python 3.8+ (matching your environment)
- **uv** (recommended) — a fast Python package installer and environment manager written in Rust. Install via `pip install uv` or see https://github.com/astral-sh/uv
- On Windows, ensure your Python architecture (e.g., 64-bit) matches the Rust toolchain target. Using a recent MSVC build chain via Visual Studio Build Tools is recommended.

**Setup with uv (recommended):**

1. Clone the repository:
```bash
git clone https://github.com/AlbertKurtz/rust_py_lib.git
cd rust_py_lib
```

2. Create a virtual environment and install with dev dependencies:
```bash
uv venv
# On Windows:
.venv\Scripts\activate
# On macOS/Linux:
source .venv/bin/activate

uv pip install -e .[dev]
```

This will compile the Rust extension and install it into your virtual environment so you can immediately `import fast_dse`.

**Alternative with pip:**

```bash
python -m venv .venv
# Activate virtual environment (see above)
pip install -e .[dev]
```

**Building a wheel manually:**

```bash
uv pip install maturin
maturin build --release
# find the .whl in target/wheels/ and install it
uv pip install target/wheels/<your_wheel_name>.whl
```

## Running the demo

After installation with dev dependencies (make sure your virtual environment is activated):

```bash
python main.py
```

This will open a plot window comparing the intensity for a spherical and cubic crystal with the parameters from the script.

## Performance notes

- `dse_optimized` uses Rayon to parallelize over `q` values and precomputes all pairwise distances for cache efficiency.
- For large crystals, the distance matrix is O(N^2) in memory and time to precompute; adjust `length` and `lattice_param` accordingly.

## Troubleshooting

- **ImportError: No module named `fast_dse`**
  - Make sure you installed the package with `uv pip install -e .[dev]`
  - Verify you're using the correct Python environment: `python -c "import sys; print(sys.executable)"`
  - Make sure your virtual environment is activated

- **Linker/build errors on Windows**
  - Ensure you have the MSVC toolchain installed via Visual Studio Build Tools
  - Verify that Rust targets your Python architecture (both should be 64-bit or both 32-bit)

- **ImportError: No module named 'matplotlib'** (when running demo)
  - Install matplotlib: `uv pip install matplotlib` or use dev dependencies: `uv pip install -e .[dev]`

## License

MIT
