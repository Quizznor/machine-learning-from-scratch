import sys
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.patches import Circle

positions = np.load('positions.npy')
distances = np.load('distances.npy')
n_vertices = len(positions)

def construct_circumcircle(point1, point2, point3):

        (x1, y1) = (point1[0], point1[1])
        (x2, y2) = (point2[0], point2[1])
        (x3, y3) = (point3[0], point3[1])

        d = (x1 - x3) * (y2 - y3) - (x2 - x3) * (y1 - y3)
        
        center_x = ((y2 - y3) * ((x1 - x3) * (x1 + x3) + (y1 - y3) * (y1 + y3))
                  - (y1 - y3) * ((x2 - x3) * (x2 + x3) + (y2 - y3) * (y2 + y3))) / (2 * d)
        center_y = ((x1 - x3) * ((x2 - x3) * (x2 + x3) + (y2 - y3) * (y2 + y3))
                  - (x2 - x3) * ((x1 - x3) * (x1 + x3) + (y1 - y3) * (y1 + y3))) / (2 * d)
        
        radius = np.sqrt((x1 - center_x)**2 + (y1 - center_y)**2)

        print('Distances:')
        print(f'\t{np.sqrt((x1 - center_x)**2 + (y1 - center_x)**2):.4f} for point1')
        print(f'\t{np.sqrt((x2 - center_x)**2 + (y2 - center_x)**2):.4f} for point2')
        print(f'\t{np.sqrt((x3 - center_x)**2 + (y3 - center_x)**2):.4f} for point3')

        return (center_x, center_y, radius)

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
    
    is_start_end_point = i in [0, index + 1]
    plt.scatter(xi, yi, 
                c = 'k' if not is_start_end_point else 'r', 
                s = 20 if not is_start_end_point else 30, 
                zorder=1
            )

    # for j, (xj, yj) in enumerate(positions[i:], i):
    #     if distances[i, j] == 0: continue
    #     plt.plot([xi, xj], [yi, yj], lw=0.8, c='gray', alpha=0.2, zorder=0)

cx, cy, r = construct_circumcircle(*[int(x) for x in sys.argv[1:]])
plt.scatter(cx, cy, c='g')
circumcircle = Circle((cx, cy), r, facecolor='none', edgecolor='k')
plt.gca().add_patch(circumcircle)

# for i in range(len(longest_path) - 1):
#     xi, yi = positions[i]
#     xf, yf = positions[i+1]

#     plt.plot([xi, xf], [yi, yf], c='r', lw=1.3)

plt.gca().set_box_aspect(1.)
plt.show()