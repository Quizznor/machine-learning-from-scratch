import osmnx
import networkx
import numpy as np
import multiprocessing

class Map():
    def __init__(self, arguments):

        self.graph = osmnx.graph_from_address(' '.join(arguments.position),
                                        network_type=arguments.type,
                                        dist=arguments.distance,
                                        dist_type="network",
        )

        # We have to manually compute the graph center point, 
        # since geocode gives wrong results for some reason...
        # Needless to say: I hate this.
        min_lat, max_lat = 91, -91
        min_lon, max_lon = 181, -181
        for (_, node) in self.graph.nodes(data=True):
            lat, lon = node['x'], node['y']

            if lat > max_lat: max_lat = lat
            elif lat < min_lat: min_lat = lat

            if lon > max_lon: max_lon = lon
            elif lon < min_lon: min_lon = lon

        self.extent_x, self.extent_y = (max_lat - min_lat), (max_lon - min_lon)
        middle_point = (min_lat + self.extent_x/2, min_lon + self.extent_y/2)
        self.graph = osmnx.routing.add_edge_speeds(self.graph)
        self.graph = osmnx.routing.add_edge_travel_times(self.graph)
        self.center_node = osmnx.nearest_nodes(self.graph, *middle_point)


    def calculate_travel_times(self):

        destinations = [node for node in self.graph.nodes() if node != self.center_node]
        origins = [self.center_node for _ in destinations]

        routes = osmnx.routing.shortest_path(self.graph, origins, destinations, weight="travel_time")

        travel_times = {}
        for r in routes:
            times = [self.graph.get_edge_data(r[i], r[i+1])[0]['travel_time'] for i in range(len(r)-1)]
            travel_times[r[-1]] = np.sum(times)

        return travel_times


    def draw(self):
        
        from matplotlib import pyplot as plt
        central_node = self.graph.nodes[self.center_node]
        plt.scatter(central_node['x'], central_node['y'], zorder=10)

        osmnx.plot_graph(self.graph, ax=plt.gca(), node_size=0)




