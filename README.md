# finite-difference-pde

PDE solvers in Rust. Heat, wave, Poisson — discretized and solved.

---

## What This Does

Finite difference methods for partial differential equations in pure Rust:

| Module | What you get |
|---|---|
| **Finite differences** | 1D/2D uniform grids, central/forward/backward differences, 2nd and 4th order |
| **Heat equation** | Forward Euler (explicit), Backward Euler (implicit), Crank-Nicolson |
| **Wave equation** | Leapfrog, Lax-Wendroff with dissipation, energy conservation tracking |
| **Poisson equation** | Jacobi, Gauss-Seidel, SOR (with optimal ω), 2D elliptic solver |
| **Advection-diffusion** | Upwind advection + central diffusion, 1D and 2D, CFL/Peclet analysis |
| **Boundary conditions** | Dirichlet, Neumann, Periodic — composable for 1D and 2D |
| **Error analysis** | L2/L∞/L1 norms, convergence order, Richardson extrapolation, total variation |

---

## Install

```toml
[dependencies]
finite-difference-pde = "0.1.0"
```

Requires **Rust 2021 edition**.

---

## Quick Start

### Heat equation (explicit)

```rust
use finite_difference_pde::{Grid1D, HeatSolver, HeatMethod, BoundaryPair1D};

let grid = Grid1D::new(0.0, 1.0, 51);
let alpha = 0.01;
let bc = BoundaryPair1D::dirichlet(0.0, 0.0);
let solver = HeatSolver::new(grid, alpha, bc, HeatMethod::ForwardEuler);

let u0: Vec<f64> = (0..solver.grid.n)
    .map(|i| (std::f64::consts::PI * solver.grid.x(i)).sin())
    .collect();

let dt = solver.max_stable_dt() * 0.8;
let u_final = solver.solve_final(&u0, dt, 200);
```

### Wave equation

```rust
use finite_difference_pde::{WaveSolver, WaveMethod, Grid1D, BoundaryPair1D};

let grid = Grid1D::new(0.0, 1.0, 101);
let solver = WaveSolver::new(grid, 1.0, BoundaryPair1D::dirichlet(0.0, 0.0), WaveMethod::Leapfrog);

let u0 = vec![/* initial displacement */];
let v0 = vec![/* initial velocity */];

let dt = solver.max_stable_dt() * 0.5;
let history = solver.solve(&u0, &v0, dt, 100);
```

### Poisson equation (2D)

```rust
use finite_difference_pde::{PoissonSolver, PoissonMethod, Grid2D, BoundaryPair2D};

let grid = Grid2D::new(0.0, 1.0, 41, 0.0, 1.0, 41);
let bc = BoundaryPair2D::dirichlet_2d(0.0, 0.0, 0.0, 0.0);
let omega = PoissonSolver::optimal_sor_omega(41, 41);
let solver = PoissonSolver::new(grid, bc, PoissonMethod::SOR(omega))
    .with_tolerance(1e-8);

let f = vec![vec![0.0; 41]; 41];
let result = solver.solve(&f, None);
println!("converged: {}, iterations: {}", result.converged, result.iterations);
```

### Error analysis

```rust
use finite_difference_pde::ErrorAnalysis;

let l2 = ErrorAnalysis::l2_error(&numerical, &exact, dx);
let linf = ErrorAnalysis::linf_error(&numerical, &exact);
let order = ErrorAnalysis::convergence_order(&errors);
let improved = ErrorAnalysis::richardson_extrapolation(&u_h, &u_h2, 2.0);
```

---

## License

MIT OR Apache-2.0
