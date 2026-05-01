import matplotlib.pyplot as plt
from itertools import product
from functools import cache
import networkx as nx
import numpy as np

class Graph(nx.Graph):

    def __init__(self, x: list, y: list,
                 popularity: list,
                 people: int):

        assert len(x) == len(y) == len(popularity), "incorrect graph shape received"

        self.n_people = people
        self.n_nodes = len(x)
        self.network_cost = 0
        super().__init__()

        for i in range(self.n_nodes):
            people_at_node = people * popularity[i] / sum(popularity)
            self.add_node(i+1, x=x[i], y=y[i], pop=people_at_node)

        for i, j in product(range(1, self.n_nodes+1), range(1, self.n_nodes+1)):
            if i == j: continue

            self.add_edge(i, j, length=self.__get_distance(i, j), grade=np.random.randint(2))
            self.network_cost += self.edges[i, j]['grade'] * self.edges[i, j]['length']


    @cache
    def __get_node_attribute(self, i, keys):

        if isinstance(keys, tuple):
            return np.array([self.nodes[i][k] for k in keys])
        else:
            return self.nodes[i][keys]
    

    @cache
    def __get_distance(self, i, j):
        
        node1 = self.__get_node_attribute(i, ('x', 'y'))
        node2 = self.__get_node_attribute(j, ('x', 'y'))
        dist = np.sqrt(np.sum((node1 - node2)**2))

        # if dist > self.max_distance: self.max_distance = dist
        # if dist < self.min_distance: self.min_distance = dist
        
        return dist
    

    def evaluate(self):

        for i in range(1, self.n_nodes+1):

            this_node_pop = self.nodes[i]['pop']
            travelers = []

            print()


    def draw(self):

        x, y = np.column_stack([self.__get_node_attribute(i, ('x', 'y')) for i in range(1, self.n_nodes+1)])
        plt.scatter(x, y, c=[self.__get_node_attribute(i, 'pop') for i in range(1, self.n_nodes+1)], s=100)

        for i, j in product(range(1, self.n_nodes+1), range(1, self.n_nodes+1)):
            if i == j: continue

            x, y = np.column_stack([self.__get_node_attribute(k, ('x', 'y')) for k in [i, j]])
            plt.plot(x, y, marker='none', zorder=0, lw=self.edges[i, j]['grade'], c='k')

        plt.title(f"{self.network_cost = :.2f}")