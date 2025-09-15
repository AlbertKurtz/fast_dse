from build_crystal import crystal, dse, dse_optimized, dse_ultra_optimized
from crystal_python import dse as dse_python, dse_ultra_optimized   
import matplotlib.pyplot as plt
import numpy as np
import timeit

def crystal_py(lattice_param: float, length: float) -> list[list[float]]:
    length_step : int = int(length / lattice_param)
    crystal : list[list[float]] = []
    for i in range(length_step):
        for j in range(length_step):
            for k in range(length_step):
                crystal.append([i * lattice_param, j * lattice_param, k * lattice_param])
    return crystal

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
    # intensity_python = dse_ultra_optimized(q_0, q_f, q_step, mycrystal)
   
    plt.plot(q_array, intensity, label="sphere")
    plt.plot(q_array, intensity_cube, label="cube")
    plt.legend()
    plt.show()
  
    # print("rust")   
    # print(timeit.timeit(lambda: dse(q_0, q_f, q_step, mycrystal), number=1))
    # print("rust (optimized)")
    # print(timeit.timeit(lambda: dse_optimized(q_0, q_f, q_step, mycrystal), number=1))
    # print("rust (ultra optimized)")
    # print(timeit.timeit(lambda: dse_ultra_optimized(q_0, q_f, q_step, mycrystal), number=1))
    # print("python (original)")
    # print(timeit.timeit(lambda: dse_python(q_0, q_f, q_step, mycrystal), number=100))
    # print("python (optimized)")
    # print(timeit.timeit(lambda: dse_ultra_optimized(q_0, q_f, q_step, mycrystal), number=1))