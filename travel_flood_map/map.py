import osmnx
import networkx
import numpy as np
import geopandas as gpd
import matplotlib.pyplot as plt
from matplotlib.path import Path
from matplotlib.patches import PathPatch
from matplotlib.ticker import FuncFormatter
from matplotlib.colors import BoundaryNorm, Normalize
from matplotlib.colorbar import ColorbarBase
from matplotlib.collections import PatchCollection
from shapely import Point, LineString, Polygon

class Map():
    def __init__(self, arguments):

        if arguments.dist is not None:
            self.graph = osmnx.graph_from_address(' '.join(arguments.location),
                                            dist=arguments.dist,
                                            dist_type="network",
                                            **self.get_filter(arguments.detail)
            )
            
        else:
            self.graph = osmnx.graph_from_place(' '.join(arguments.location),
                                                **self.get_filter(arguments.detail)
            )

        if arguments.start is not None:
            geocode = osmnx.geocoder.geocode(' '.join(arguments.start))
            self.start_node = osmnx.nearest_nodes(self.graph, *geocode[::-1])
        else:
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

            middle_point = (min_lat + (max_lat - min_lat)/2, min_lon + (max_lon - min_lon)/2)
            self.start_node = osmnx.nearest_nodes(self.graph, *middle_point)

        self.graph = osmnx.routing.add_edge_speeds(self.graph)
        self.graph = osmnx.routing.add_edge_travel_times(self.graph)
        self.args = arguments


    def get_isochrone(self, travel_time):

        subgraph = networkx.ego_graph(self.graph, self.start_node,
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

        n = nodes_gdf.buffer(6e-5).geometry
        e = gpd.GeoSeries(edge_lines).buffer(6e-5).geometry
        all_gs = list(n) + list(e)
        polygon = gpd.GeoSeries(all_gs).unary_union
        
        return Polygon(polygon.exterior)
    
    
    def draw(self, **kwargs):

        cmap = kwargs.get("cmap", plt.cm.plasma)
        n_points = kwargs.get("n_points", 6)
        max_time = kwargs.get("max_time", False)

        plt.rcParams["font.family"] = 'Palatino'
        plt.rcParams["text.usetex"] = True

        fig, (ax, cax) = plt.subplots(1, 2, width_ratios = [1, 0.03])
        
        if not max_time:
            travel_times = networkx.single_source_dijkstra_path_length(self.graph, 
                                                                    self.start_node, 
                                                                    weight='travel_time')

            max_travel_time_seconds = list(travel_times.values())[-1] 
            travel_times = np.linspace(0, max_travel_time_seconds,
                                    n_points,
                                    dtype=int)[1:]
        else:
            travel_times = np.linspace(0, max_time,
                                       n_points,
                                       dtype=int)[1:]

        norm = Normalize(np.min(travel_times), np.max(travel_times))
        colors = [cmap(norm(x)) for x in travel_times]

        for t, c in zip(travel_times[::-1], colors[::-1]):
            polygon = self.get_isochrone(t)
            self.plot_isochrone(ax, polygon, color=c, alpha=0.3, zorder=10)

        ColorbarBase(cax, cmap=cmap, orientation='vertical', label="Traveling time",
                     norm = BoundaryNorm([0] + list(travel_times), cmap.N),
                     format=FuncFormatter(self.format_label))

        title = f"{' '.join(self.args.start)}, " if self.args.start is not None else ""
        title += ' '.join(self.args.location)
        title += f" - {self.args.dist}m" if self.args.dist is not None else "" 
        fig.suptitle(title, fontsize=20)
        osmnx.plot_graph(self.graph, ax=ax, node_size=0)

        fig.savefig(f"{'_'.join(self.args.location)}_{self.args.dist if self.args.dist is not None else ''}_{len(travel_times)}levels.png")


    @staticmethod
    def plot_isochrone(ax, polygon, **kwargs):

        path = Path.make_compound_path(
            Path(np.asarray(polygon.exterior.coords)[:, :2]),
            *[Path(np.asarray(ring.coords)[:, :2]) for ring in polygon.interiors])

        patch = PathPatch(path, **kwargs)
        collection = PatchCollection([patch], **kwargs)
        ax.add_collection(collection, autolim=True)

    @staticmethod
    def format_label(x, _):

        return_str, remaining = "", x

        if (days := remaining // (60 * 60 * 24)):
            remaining %= (60 * 60 * 24)
            return_str += f"{days}d "
        if (hours := remaining // (60 * 60)):
            remaining %= (60 * 60)
            return_str += f"{hours}h "
        if (minutes := remaining // 60):
            remaining %= 60
            return_str += f"{minutes}m "
        
        return_str += f"{remaining}s"
        return return_str
    
    @staticmethod
    def get_filter(lod):
        
        filter_dict = [
            {'custom_filter' : '["highway"~"motorway|trunk|primary|motorway_link|trunk_link|primary_link"]'}, # major roads only
            {'network_type' : 'bike'},                                                                        # no footpaths
            {'network_type' : 'all'},                                                                         # everything
        ]

        return filter_dict[lod - 1]

