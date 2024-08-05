import osmnx
import networkx
import numpy as np
import geopandas as gpd
import matplotlib.pyplot as plt
from matplotlib.path import Path
from matplotlib.colors import Normalize
from matplotlib.patches import PathPatch
from matplotlib.colorbar import ColorbarBase
from matplotlib.collections import PatchCollection
from shapely import Point, LineString, Polygon

class Map():
    def __init__(self, arguments):

        self.graph = osmnx.graph_from_address(' '.join(arguments.position),
                                        network_type=arguments.type,
                                        dist=arguments.distance,
                                        dist_type="network",
        )

        self.args = arguments

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


    def get_isochrone(self, travel_time):

        subgraph = networkx.ego_graph(self.graph, self.center_node,
                                      radius = travel_time,
                                      distance = 'travel_time')
        
        node_points = [Point((data['x'], data['y'])) for node, data in subgraph.nodes(data=True)]
        nodes_gdf = gpd.GeoDataFrame({'id': subgraph.nodes()}, geometry=node_points)
        nodes_gdf = nodes_gdf.set_index('id')

        edge_lines = []
        for n_fr, n_to in subgraph.edges():
            f = nodes_gdf.loc[n_fr].geometry
            t = nodes_gdf.loc[n_to].geometry
            edge_lines.append(LineString([f,t]))

        edges_gdf = gpd.GeoDataFrame(geometry=edge_lines)
        polygon = edges_gdf.buffer(6e-5).unary_union
        
        return Polygon(polygon.exterior)
    
    
    def draw(self, **kwargs):

        cmap = kwargs.get("cmap", plt.cm.plasma)
        n_points = kwargs.get("n_points", 6)

        fig, (ax, cax) = plt.subplots(2, 1, height_ratios = [1, 0.03])
        travel_times = networkx.single_source_dijkstra_path_length(self.graph, 
                                                                   self.center_node, 
                                                                   weight='travel_time')
        
        travel_times = np.linspace(0, list(travel_times.values())[-1],
                                   n_points,
                                   dtype=int)[1:]

        norm = Normalize(travel_times[0], travel_times[-1])
        colors = [cmap(norm(x)) for x in travel_times]

        for t, c in zip(travel_times[::-1], colors[::-1]):
            polygon = self.get_isochrone(t)
            self.plot_isochrone(ax, polygon, color=c, alpha=0.2, zorder=10)

        ColorbarBase(cax, cmap=cmap, norm=norm, orientation='horizontal', label="Traveling time")
        osmnx.plot_graph(self.graph, ax=ax, node_size=0)

        fig.savefig(f"{'_'.join(self.args.position)}_{self.args.distance}_{self.args.type}_{len(travel_times)}levels.png")


    @staticmethod
    def plot_isochrone(ax, polygon, **kwargs):

        path = Path.make_compound_path(
            Path(np.asarray(polygon.exterior.coords)[:, :2]),
            *[Path(np.asarray(ring.coords)[:, :2]) for ring in polygon.interiors])

        patch = PathPatch(path, **kwargs)
        collection = PatchCollection([patch], **kwargs)
        ax.add_collection(collection, autolim=True)