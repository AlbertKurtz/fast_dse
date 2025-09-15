from build_crystal import crystal, dse_optimized
import matplotlib.pyplot as plt
import numpy as np

if __name__ == "__main__":
    lattice_param = 3.89
    length = 30
    sphere_crystal = crystal("sphere", lattice_param, length)
    cube_crystal = crystal("cube", lattice_param, length)
    q_0 = 1
    q_f = 15
    q_step = 0.1
    q_array = np.arange(q_0, q_f, q_step)
    intensity = dse_optimized(q_0, q_f, q_step, sphere_crystal)
    intensity_cube = dse_optimized(q_0, q_f, q_step, cube_crystal)

    plt.plot(q_array, intensity, label="sphere")
    plt.plot(q_array, intensity_cube, label="cube")
    plt.legend()
    plt.show()
  