import argparse
from map import Map

parser = argparse.ArgumentParser(description='Build a flushmap of travel times')
parser.add_argument('position',
                    nargs="+",
                    help="Starting point for the flush map")
parser.add_argument('-n',
                    default=1000,
                    type=int,
                    help="how many routes to evaluate")
parser.add_argument('--type',
                    '-t',
                    default="all",
                    choices=["all", "drive", "bike", "walk"],
                    help="Specify the type of transportation")
parser.add_argument('--distance',
                    '-d',
                    type=int,
                    default=10000,
                    help="Build the flood map up to this travel distance")

if __name__ == "__main__":

    flood_map = Map(parser.parse_args())

    # print(flood_map.graph.node[])

    routes = flood_map.solve()

    # for route in routes:
    #     print(route)