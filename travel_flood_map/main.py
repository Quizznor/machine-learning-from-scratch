import argparse
from map import Map

parser = argparse.ArgumentParser(description='Build a flushmap of travel times')
parser.add_argument('location',
                    nargs="+",
                    help="the location for which isochrones are calculated")
parser.add_argument('--start',
                    nargs="+",
                    default=None,
                    help="Starting point for route calculations")
parser.add_argument('--dist',
                    '-d',
                    type=int,
                    default=None,
                    help="Build the flood map up to this travel distance")
parser.add_argument("--detail",
                    "-lod",
                    type=int,
                    default=3,
                    choices=[1, 2, 3],
                    help="Mapping detail, 3 - everything, 2 - major/minor roads, 1 - major roads only")

if __name__ == "__main__":

    flood_map = Map(parser.parse_args())
    flood_map.draw()