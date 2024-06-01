import numpy as np
import matplotlib.pyplot as plt

positions = np.load('positions.npy')
distances = np.load('distances.npy')
n_vertices = len(positions)

for i, ((xi, yi), edges) in enumerate(zip(positions, distances), 0):
    
    is_start_end_point = i % (n_vertices - 1) != 0
    plt.scatter(xi, yi, c='k' if is_start_end_point else 'r', s=20, zorder=1)
    for j, (xj, yj) in enumerate(positions[i:], i):
        if distances[i, j] == 0: continue

        plt.plot([xi, xj], [yi, yj], lw=0.8, c='gray', alpha=0.2, zorder=0)

# solution = np.load('solution.npy') # TODO

plt.show()