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
        print(f'\t{np.sqrt((x1 - center_x)**2 + (y1 - center_y)**2):.4f} for point1')
        print(f'\t{np.sqrt((x2 - center_x)**2 + (y2 - center_y)**2):.4f} for point2')
        print(f'\t{np.sqrt((x3 - center_x)**2 + (y3 - center_y)**2):.4f} for point3')

        return (center_x, center_y, radius)

def read_paths(target : str) -> list[int] :
    
    longest_index, longest_distance = -1, -np.inf

    with open(target, 'r') as f:
        for i, line in enumerate(f.readlines()):
            line = line.strip().split(',')
            path = [int(x) for x in line[:-1]]
            distance = float(line[-1])
            
            if distance > longest_distance:
                longest_distance = distance
                longest_path = path
                longest_index = i

    return longest_path, longest_index

def point_is_inside_circumcircle(point, cx, cy, r):
    return np.sqrt( (point[0] - cx)**2 + (point[1] - cy)**2 ) <= r


longest_path, index = read_paths('minimum_distance_paths.txt')

if len(sys.argv[1:]) > 0:
     
    selected_points = np.array([positions[int(x)] for x in sys.argv[1:]])
    cx, cy, r = construct_circumcircle(*selected_points)
    circumcircle = Circle((cx, cy), r, facecolor='none', edgecolor='k')
    plt.gca().add_patch(circumcircle)

    colors = ["green" if point_is_inside_circumcircle(p, cx, cy, r) else "red" for p in positions]
    plt.scatter(positions[:, 0], positions[:, 1], 
            c = colors,
            zorder=1
        )

    plt.scatter(selected_points[:, 0], selected_points[:, 1], c='blue', s=70)

else:

    print(f"(Longest) shortes path from 0 -> {index}: {longest_path}")

    for i, node in enumerate(longest_path[:-1]):
        plt.scatter(*positions[node], c='b', zorder=9, s=70)
        
        xi, yi = positions[node]
        xf, yf = positions[longest_path[i+1]]
        plt.plot([xi, xf], [yi, yf], c='r', lw=1.3, zorder=0)

    for i, ((xi, yi), edges) in enumerate(zip(positions, distances), 0):
        
        is_start_end_point = i in [0, index]
        plt.text(xi, yi, str(i), 
                 horizontalalignment='center',
                 verticalalignment='center',
                 fontsize=7,
                 color='w',
                 zorder=10)
        plt.scatter(xi, yi, 
                    c = 'k' if not is_start_end_point else 'r', 
                    s = 70, 
                    zorder=1
                )

        for j, (xj, yj) in enumerate(positions[i:], i):
            if distances[i, j] == 0: continue
            plt.plot([xi, xj], [yi, yj], lw=0.8, c='gray', alpha=0.2, zorder=0)
            
plt.gca().set_box_aspect(1.)
plt.xlim(-0.05, 1.05)
plt.ylim(-0.05, 1.05)
plt.savefig("graph.png")