import numpy as np
import matplotlib.pyplot as plt

positions = np.load('positions.npy')
distances = np.load('distances.npy')
n_vertices = len(positions)

def read_minimum_paths(target : str) -> list[int] :
    
    longest_index, longest_len = -1, 0

    with open(target, 'r') as f:
        for i, line in enumerate(f.readlines()):
            line = [int(x) for x in line.strip().split(',')[1:]]
            
            if len(line) > longest_len:
                longest_len = len(line)
                longest_path = line
                longest_index = i

    return longest_path, longest_index

longest_path, index = read_minimum_paths('minimum_distance_paths.txt')

for i, ((xi, yi), edges) in enumerate(zip(positions, distances), 0):
    
    is_start_end_point = i in [0, index]
    plt.scatter(xi, yi, 
                c = 'k' if not is_start_end_point else 'r', 
                s = 20 if not is_start_end_point else 30, 
                zorder=1
            )

    for j, (xj, yj) in enumerate(positions[i:], i):
        if distances[i, j] == 0: continue

        plt.plot([xi, xj], [yi, yj], lw=0.8, c='gray', alpha=0.2, zorder=0)

for i in range(len(longest_path) - 1):
    xi, yi = positions[i]
    xf, yf = positions[i+1]

    plt.plot([xi, xf], [yi, yf], c='r', lw=1.3)

plt.show()