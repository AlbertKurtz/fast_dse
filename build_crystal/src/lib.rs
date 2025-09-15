use pyo3::prelude::*;
use rayon::prelude::*;



/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn build_crystal(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(crystal, m)?)?;
    m.add_function(wrap_pyfunction!(dse, m)?)?;
    m.add_function(wrap_pyfunction!(dse_optimized, m)?)?;
    m.add_function(wrap_pyfunction!(dse_ultra_optimized, m)?)?;
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

fn intensity_at_q_optimized(crystal: &[Vec<f64>], q: f64) -> f64 {
    let n = crystal.len();
    let mut intensity = 0.0;
    
    // Pre-calculate all distance squares to avoid redundant sqrt calculations
    let mut distances_sq = Vec::with_capacity(n * n);
    
    for i in 0..n {
        for j in 0..n {
            let dx = crystal[i][0] - crystal[j][0];
            let dy = crystal[i][1] - crystal[j][1];
            let dz = crystal[i][2] - crystal[j][2];
            distances_sq.push(dx * dx + dy * dy + dz * dz);
        }
    }
    
    // Vectorized calculation using iterator and parallel processing
    intensity = distances_sq
        .par_iter()
        .map(|&dist_sq| intensity_point_optimized(q, dist_sq))
        .sum();
    
    intensity
}

fn intensity_at_q_ultra_optimized(crystal: &[Vec<f64>], q: f64) -> f64 {
    let n = crystal.len();
    
    // Use parallel iteration over pairs
    (0..n)
        .into_par_iter()
        .map(|i| {
            let mut row_intensity = 0.0;
            for j in 0..n {
                let dx = crystal[i][0] - crystal[j][0];
                let dy = crystal[i][1] - crystal[j][1];
                let dz = crystal[i][2] - crystal[j][2];
                let distance_sq = dx * dx + dy * dy + dz * dz;
                
                if distance_sq == 0.0 {
                    row_intensity += 1.0;
                } else {
                    let distance = distance_sq.sqrt();
                    let qd = q * distance;
                    row_intensity += qd.sin() / qd;
                }
            }
            row_intensity
        })
        .sum()
}

#[pyfunction]
fn dse(min_q : f64, max_q : f64, q_step : f64, crystal : Vec<Vec<f64>>) -> PyResult<Vec<f64>> {
    let n_points : usize= ((max_q - min_q) / q_step).floor() as usize;
    let mut intensity : Vec<f64> = vec![0.0; n_points];
    let mut q = min_q;
    let mut index = 0;
    while index < n_points {
        intensity[index] = intensity_at_q(&crystal, q);
        index+=1;
        q+=q_step;
    }
    Ok(intensity)
}

#[pyfunction]
fn dse_optimized(min_q: f64, max_q: f64, q_step: f64, crystal: Vec<Vec<f64>>) -> PyResult<Vec<f64>> {
    let n_points = ((max_q - min_q) / q_step).floor() as usize;
    let mut intensity = vec![0.0; n_points];
    
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
    
    intensity = q_values
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

#[pyfunction]
fn dse_ultra_optimized(min_q: f64, max_q: f64, q_step: f64, crystal: Vec<Vec<f64>>) -> PyResult<Vec<f64>> {
    let n_points = ((max_q - min_q) / q_step).floor() as usize;
    let n = crystal.len();
    
    // Pre-calculate distance matrix once
    let mut distances_sq = Vec::with_capacity(n * n);
    for i in 0..n {
        for j in 0..n {
            let dx = crystal[i][0] - crystal[j][0];
            let dy = crystal[i][1] - crystal[j][1]; 
            let dz = crystal[i][2] - crystal[j][2];
            distances_sq.push(dx * dx + dy * dy + dz * dz);
        }
    }
    
    // Parallel computation over both q values and distance pairs
    let intensity: Vec<f64> = (0..n_points)
        .into_par_iter()
        .map(|i| {
            let q = min_q + i as f64 * q_step;
            distances_sq
                .par_iter()
                .map(|&dist_sq| {
                    if dist_sq == 0.0 {
                        1.0
                    } else {
                        let distance = dist_sq.sqrt();
                        let qd = q * distance;
                        qd.sin() / qd
                    }
                })
                .sum()
        })
        .collect();
    
    Ok(intensity)
}

fn intensity_point(q : f64, r_1 : &[f64], r_2 : &[f64]) -> f64 {
    let distance = ((r_1[0] - r_2[0]).powi(2) + (r_1[1] - r_2[1]).powi(2) + (r_1[2] - r_2[2]).powi(2)).sqrt();
    if distance == 0.0 {
        return 1.0;
    }
    return ((q * distance).sin())/(q * distance);
}

fn intensity_at_q(crystal : &Vec<Vec<f64>>, q : f64) -> f64 {
    let mut intensity : f64 = 0.0;
    for i in 0..crystal.len() {
        for j in 0..crystal.len() {
            intensity += intensity_point(q, crystal[i].as_slice(), crystal[j].as_slice());
        }
    }
    return intensity;
}
