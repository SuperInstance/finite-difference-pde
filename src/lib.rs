//! # finite-difference-pde
//!
//! Finite difference PDE solvers in Rust.
//!
//! Provides finite difference schemes, solvers for the heat, wave, Poisson,
//! and advection-diffusion equations, flexible boundary conditions, and error
//! analysis utilities.

pub mod boundary;
pub mod error_analysis;
pub mod finite_diff;
pub mod heat;
pub mod wave;
pub mod poisson;
pub mod advection_diffusion;

pub use boundary::{BoundaryCondition, BoundaryPair1D, BoundaryPair2D};
pub use finite_diff::{Grid1D, Grid2D, FiniteDiff};
pub use heat::HeatSolver;
pub use wave::WaveSolver;
pub use poisson::PoissonSolver;
pub use advection_diffusion::AdvectionDiffusionSolver;
pub use error_analysis::ErrorAnalysis;
