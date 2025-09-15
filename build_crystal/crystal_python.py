import numpy as np

def intensity_point(q : float, r_1 : np.ndarray, r_2 : np.ndarray) -> float:
    distance = np.linalg.norm(r_1 - r_2)
    if distance == 0.0:
        return 1.0
    return np.sin(q * distance) / (q * distance)

def intensity_at_q(mycrystal : np.ndarray, q : float) -> float:
    intensity = 0.0
    for i in range(len(mycrystal)):
        for j in range(len(mycrystal)):
            intensity += intensity_point(q, np.array(mycrystal[i]), np.array(mycrystal[j]))
    return intensity


def dse(q_0 : float, q_f : float, q_step : float, mycrystal : np.ndarray) -> np.ndarray:
    n_points = int((q_f - q_0) / q_step)
    intensity = np.zeros(n_points)
    q = np.arange(q_0, q_0 + n_points * q_step, q_step)
    for i, q_i in enumerate(q):
        intensity[i] = intensity_at_q(mycrystal, float(q_i))
    return intensity

def dse_ultra_optimized(min_q: float, max_q: float, q_step: float, crystal: np.ndarray) -> np.ndarray:
    """Ultra-optimized version that vectorizes over both atoms and q values."""
    crystal = np.asarray(crystal)
    
    # Create q array
    n_points = int((max_q - min_q) / q_step)
    q_values = np.linspace(min_q, min_q + (n_points - 1) * q_step, n_points)
    
    # Pre-calculate distance matrix using optimized method
    # This computes ||r_i - r_j||^2 for all pairs using the identity:
    # ||a - b||^2 = ||a||^2 + ||b||^2 - 2*a·b
    r_sq = np.einsum('ij,ij->i', crystal, crystal)  # ||r_i||^2 for each atom
    distances_sq = r_sq[:, None] + r_sq[None, :] - 2 * np.dot(crystal, crystal.T)
    
    # Ensure no negative values due to floating point errors
    distances_sq = np.maximum(distances_sq, 0.0)
    distances = np.sqrt(distances_sq)
    
    # Vectorize over all q values at once using broadcasting
    # q_values has shape (n_points,)
    # distances has shape (n_atoms, n_atoms)  
    # qd will have shape (n_points, n_atoms, n_atoms)
    qd = q_values[:, None, None] * distances[None, :, :]
    
    # Calculate sinc for all q and distance combinations
    with np.errstate(divide='ignore', invalid='ignore'):
        intensity_matrix = np.where(distances[None, :, :] == 0.0, 1.0, np.sin(qd) / qd)
    
    # Sum over atom pairs for each q value
    intensity = np.sum(intensity_matrix, axis=(1, 2))
    
    return intensity