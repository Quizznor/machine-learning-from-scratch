from matplotlib.gridspec import GridSpec
from cartopy.crs import Robinson, Geodetic
from astropy import coordinates, units
from matplotlib.widgets import Slider
import matplotlib.pyplot as plt
import scienceplots
from . import np
import PIL

from .constants import *
from .orbit import Orbit


plt.style.use(['science'])

class Window(plt.Figure):

    def __init__(self) -> None:

        self.fig = plt.figure()
        manager = plt.get_current_fig_manager()
        manager.full_screen_toggle()

        self.__gs = GridSpec(
            6, 2,
            self.fig,
            height_ratios=[0.95, 0.02, 0.02, 0.02, 0.02, 0.02],
        )

        self.view_3d = self.__init_3d_view(blue_marble=False)
        self.view_2d = self.__init_2d_view()
        self.config = self.__init_config()

        self.orbit = Orbit(orbital_elements = {
            'a' : 2 * EARTH_RADIUS_KM,      # semi-major axis (km)
            'e': 0.3,                       # eccentricity (0, 1)
            'omega' : np.pi/4,              # RA of ascending node (rad)
            'i': 30 * np.pi/180,            # inclination (rad)
            }
        )

        ellipse = self.orbit._get_ellipse()
        test = ellipse(np.linspace(0, 2*np.pi, 100))

        self.view_3d.plot(*test, lw=2)

        plt.subplots_adjust(wspace=0)
        plt.show()


    def __init_3d_view(self, blue_marble: bool=False) -> plt.Axes:

        view_3d = self.fig.add_subplot(self.__gs[0, 0], projection='3d')
        view_3d.set_box_aspect((1, 1, 1))

        if blue_marble:
            blue_marble = np.array(PIL.Image.open('./bluemarble.jpg')) / 256.

            lons = np.linspace(-180, 180, blue_marble.shape[1]) * np.pi/180 
            lats = np.linspace(-90, 90, blue_marble.shape[0])[::-1] * np.pi/180 

            x = np.outer(np.cos(lons), np.cos(lats)).T
            y = np.outer(np.sin(lons), np.cos(lats)).T
            z = np.outer(np.ones_like(lons), np.sin(lats)).T
        else:
            lons = np.linspace(-180, 180, 100) * np.pi/180 
            lats = np.linspace(-90, 90, 100)[::-1] * np.pi/180 

            x = np.outer(np.cos(lons), np.cos(lats)).T
            y = np.outer(np.sin(lons), np.cos(lats)).T
            z = np.outer(np.ones_like(lons), np.sin(lats)).T

        view_3d.plot_surface(EARTH_RADIUS_KM * x, 
                             EARTH_RADIUS_KM * y, 
                             EARTH_RADIUS_KM * z, 
                             rstride=4, cstride=4, 
                             facecolor = "b",
                             edgecolors="k", 
                             linewidths=0.1,
                             alpha=0.04)

        view_3d.set_xlim(-AXIS_3D_LIMIT_KM, AXIS_3D_LIMIT_KM)
        view_3d.set_ylim(-AXIS_3D_LIMIT_KM, AXIS_3D_LIMIT_KM)
        view_3d.set_zlim(-AXIS_3D_LIMIT_KM, AXIS_3D_LIMIT_KM)
        view_3d.set_box_aspect((1, 1, 1))
        view_3d.set_axis_off()

        return view_3d


    def __init_2d_view(self) -> plt.Axes:

        view_2d = self.fig.add_subplot(self.__gs[0, 1], projection=Robinson())
        view_2d.coastlines(resolution='50m', lw=0.7, animated=False)
        view_2d.gridlines(animated=False)
        view_2d.set_global()

        return view_2d
    

    def __init_config(self) -> dict:

        config = {}

        for k, param in enumerate(["a", "e", "i", r"\Omega", r"\omega"], 1):
            config[param] = Slider(self.fig.add_subplot(self.__gs[k, 0]), 
                                   f"${param}$", **PARAM_LIMITS[param])