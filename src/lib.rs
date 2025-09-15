use pyo3::prelude::*;
use rayon::prelude::*;



/// A Python module implemented in Rust.
#[pymodule]
fn build_crystal(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(crystal, m)?)?;
    m.add_function(wrap_pyfunction!(dse_optimized, m)?)?;
    Ok(())
}

#[pyfunction]
fn crystal(shape : &str, lattice_param: f64, length : f64) -> PyResult<Vec<Vec<f64>>> {
    let length_step : usize = (length / lattice_param).floor() as usize;
    let mut crystal : Vec<Vec<f64>> = Vec::new();
    match shape {
        "cube" => {
            for i in 0..length_step {
                for j in 0..length_step {
                    for k in 0..length_step {
                        crystal.push(vec![i as f64 * lattice_param, j as f64 * lattice_param, k as f64 * lattice_param]);
                    }
                }
            }
        }   
        "sphere" => {
            let radius = length / 2.0;
            let center = vec![radius, radius, radius];
            for i in 0..length_step {
                for j in 0..length_step {
                    for k in 0..length_step {
                        let point = vec![i as f64 * lattice_param, j as f64 * lattice_param, k as f64 * lattice_param];
                        let distance = (point[0] - center[0]).powi(2) + (point[1] - center[1]).powi(2) + (point[2] - center[2]).powi(2);
                        if distance <= radius.powi(2) {
                            crystal.push(point);
                        }
                    }
                }
            }
        }
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Unknown shape: '{}'. Supported shapes: 'cube', 'sphere'", shape)
            ));
        }
    }
    Ok(crystal)
}

#[inline(always)]
fn intensity_point_optimized(q: f64, distance_sq: f64) -> f64 {
    if distance_sq == 0.0 {
        return 1.0;
    }
    let distance = distance_sq.sqrt();
    let qd = q * distance;
    qd.sin() / qd
}


#[pyfunction]
fn dse_optimized(min_q: f64, max_q: f64, q_step: f64, crystal: Vec<Vec<f64>>) -> PyResult<Vec<f64>> {
    let n_points = ((max_q - min_q) / q_step).floor() as usize;
    
    
    // Pre-calculate distance matrix once
    let n = crystal.len();
    let mut distances_sq = Vec::with_capacity(n * n);
    
    for i in 0..n {
        for j in 0..n {
            let dx = crystal[i][0] - crystal[j][0];
            let dy = crystal[i][1] - crystal[j][1];
            let dz = crystal[i][2] - crystal[j][2];
            distances_sq.push(dx * dx + dy * dy + dz * dz);
        }
    }
    
    // Parallel computation over q values
    let q_values: Vec<f64> = (0..n_points)
        .map(|i| min_q + i as f64 * q_step)
        .collect();
    
    let intensity = q_values
        .par_iter()
        .map(|&q| {
            distances_sq
                .iter()
                .map(|&dist_sq| intensity_point_optimized(q, dist_sq))
                .sum()
        })
        .collect();
    
    Ok(intensity)
}
