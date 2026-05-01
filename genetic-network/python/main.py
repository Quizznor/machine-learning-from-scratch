import matplotlib.pyplot as plt
from src import graph
import numpy as np

N_NODES = 20
N_PEOPLE = 10_000

x = np.random.uniform(size=N_NODES)
y = np.random.uniform(size=N_NODES)
popularity = np.random.uniform(size=N_NODES)

# connections = np.random.uniform(size=(N_NODES, N_NODES))

test = graph.Graph(
    x, y,
    popularity,
    N_PEOPLE,
)

test.evaluate()
# test.draw()
# plt.show()