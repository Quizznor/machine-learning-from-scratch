import sys
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.patches import Circle

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

def draw_circle(points, positions, distances, n_vertices):
    selected_points = np.array([positions[int(x)] for x in points])
    cx, cy, r = construct_circumcircle(*selected_points)
    circumcircle = Circle((cx, cy), r, facecolor='none', edgecolor='k')
    plt.gca().add_patch(circumcircle)

    colors = ["green" if point_is_inside_circumcircle(p, cx, cy, r) else "red" for p in positions]
    plt.scatter(positions[:, 0], positions[:, 1], 
            c = colors,
            zorder=1
        )

    plt.scatter(selected_points[:, 0], selected_points[:, 1], c='blue', s=70)

def draw_path(positions, distances, n_vertices, longest_path, index):

    print(f"(Longest) shortes path from 0 -> {index}: {longest_path}")

    for i, ((xi, yi), edges) in enumerate(zip(positions, distances), 0):
        
        plt.text(xi, yi, str(i), 
                        horizontalalignment='center',
                        verticalalignment='center',
                        fontsize=4,
                        color='w',
                        zorder=10)

        is_start_end_point = i in [0, index]
        plt.scatter(xi, yi, 
                    c = 'k' if not is_start_end_point else 'r', 
                    s = 70, 
                    zorder=1
                )

        for j, (xj, yj) in enumerate(positions[i:], i):
            if distances[i, j] == 0: continue
            plt.plot([xi, xj], [yi, yj], lw=0.8, c='gray', alpha=0.4, zorder=0)

    for i, node in enumerate(longest_path[:-1]):
        plt.scatter(*positions[node], c='b', zorder=9, s=70)
        
        xi, yi = positions[node]
        xf, yf = positions[longest_path[i+1]]
        plt.plot([xi, xf], [yi, yf], c='r', lw=1.3, zorder=0)

def draw_floodmap(positions, distances, n_vertices, min_distance_paths):

    import matplotlib.cm as cm

    drive_times = np.zeros(100)
    with open(min_distance_paths, 'r') as paths:
        for i, line in enumerate(paths.readlines()):
            drive_times[i] = float(line.strip().split(',')[-1])

    drive_times /= np.max(drive_times)

    for i, ((xi, yi), edges) in enumerate(zip(positions, distances), 0):

        plt.text(xi, yi, str(i), 
                horizontalalignment='center',
                verticalalignment='center',
                fontsize=4,
                color=cm.Greys(drive_times[i]),
                zorder=10)
        
        is_start_end_point = i in [0, index]
        plt.scatter(xi, yi, 
                    color = cm.plasma(drive_times[i]), 
                    s = 70, 
                    zorder=1
                )

        for j, (xj, yj) in enumerate(positions[i:], i):
            if distances[i, j] == 0: continue
            plt.plot([xi, xj], [yi, yj], lw=0.8, c='gray', alpha=0.4, zorder=0)

if __name__ == "__main__":
    longest_path, index = read_paths('minimum_distance_paths.txt')
    positions = np.load('positions.npy')
    distances = np.load('distances.npy')
    n_vertices = len(positions)

    if len(sys.argv[1:]) > 1:
        draw_circle(sys.argv[1:], positions, distances, n_vertices)
    elif len(sys.argv[1:]) == 1:
        draw_floodmap(positions, distances, n_vertices, 'minimum_distance_paths.txt')   
    else:
        draw_path(positions, distances, n_vertices, longest_path, index)

    plt.gca().set_box_aspect(1.)
    plt.xlim(-0.05, 1.05)
    plt.ylim(-0.05, 1.05)
    plt.axis('off')
    plt.tight_layout()
    plt.savefig("graph.png", dpi=1200)