//! Gregory Galperin's 1995 Elastic Collision Billiard Simulator
//!
//! Computes digits of Pi using Hamiltonian dynamics of two colliding blocks and a rigid wall.
//! When mass ratio M/m = 100^N, total number of collisions before separation is exactly floor(pi * 10^N).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GalperinResult {
    pub mass_ratio: f64,
    pub collisions: u64,
    pub wall_collisions: u64,
    pub block_collisions: u64,
    pub energy_error: f64,
    pub estimated_pi: f64,
}

/// Exact event-driven simulation of 1D elastic collisions between two blocks and a wall at x = 0
///
/// Block 1: mass m = 1.0, initial position x1 = 1.0, initial velocity v1 = 0.0
/// Block 2: mass M = mass_ratio, initial position x2 = 2.0, initial velocity v2 = -1.0
pub fn simulate_galperin(mass_ratio: f64, max_collisions: u64) -> GalperinResult {
    let m1 = 1.0f64;
    let m2 = mass_ratio.max(1.0);

    let mut x1 = 1.0f64;
    let mut v1 = 0.0f64;
    let mut x2 = 2.0f64;
    let mut v2 = -1.0f64;

    let initial_energy = 0.5 * m1 * v1 * v1 + 0.5 * m2 * v2 * v2;

    let mut wall_collisions = 0u64;
    let mut block_collisions = 0u64;

    while wall_collisions + block_collisions < max_collisions {
        // Termination condition: Block 2 moves to the right faster than Block 1, and Block 1 is moving right
        // Both move away from the wall and each other forever.
        if v1 >= 0.0 && v2 >= v1 {
            break;
        }

        // Calculate time to next wall collision: x1 + v1 * dt = 0
        let t_wall = if v1 < 0.0 {
            Some(-x1 / v1)
        } else {
            None
        };

        // Calculate time to next inter-block collision: x1 + v1 * dt = x2 + v2 * dt
        let t_block = if v1 > v2 {
            Some((x2 - x1) / (v1 - v2))
        } else {
            None
        };

        match (t_wall, t_block) {
            (Some(tw), Some(tb)) => {
                if tw < tb {
                    // Wall collision happens first
                    x1 = 0.0;
                    x2 += v2 * tw;
                    v1 = -v1;
                    wall_collisions += 1;
                } else {
                    // Block collision happens first
                    x1 += v1 * tb;
                    x2 += v2 * tb;
                    let new_v1 = ((m1 - m2) * v1 + 2.0 * m2 * v2) / (m1 + m2);
                    let new_v2 = (2.0 * m1 * v1 + (m2 - m1) * v2) / (m1 + m2);
                    v1 = new_v1;
                    v2 = new_v2;
                    block_collisions += 1;
                }
            }
            (Some(tw), None) => {
                x1 = 0.0;
                x2 += v2 * tw;
                v1 = -v1;
                wall_collisions += 1;
            }
            (None, Some(tb)) => {
                x1 += v1 * tb;
                x2 += v2 * tb;
                let new_v1 = ((m1 - m2) * v1 + 2.0 * m2 * v2) / (m1 + m2);
                let new_v2 = (2.0 * m1 * v1 + (m2 - m1) * v2) / (m1 + m2);
                v1 = new_v1;
                v2 = new_v2;
                block_collisions += 1;
            }
            (None, None) => break,
        }
    }

    let final_energy = 0.5 * m1 * v1 * v1 + 0.5 * m2 * v2 * v2;
    let energy_error = ((final_energy - initial_energy) / initial_energy).abs();

    let total_collisions = wall_collisions + block_collisions;
    // For M/m = 100^N, total_collisions = floor(pi * 10^N)
    // So estimated_pi = total_collisions / (sqrt(mass_ratio))
    let scale = mass_ratio.sqrt();
    let estimated_pi = (total_collisions as f64) / scale;

    GalperinResult {
        mass_ratio,
        collisions: total_collisions,
        wall_collisions,
        block_collisions,
        energy_error,
        estimated_pi,
    }
}
