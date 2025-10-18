from matplotlib.gridspec import GridSpec
from cartopy.crs import Robinson, Geodetic
from astropy import coordinates, units
from matplotlib.widgets import Slider
import matplotlib.pyplot as plt
import scienceplots
import PIL

from .constants import *
from .orbit import Orbit
from . import np


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
        
        self.config = self.__init_config()
        self.orbit = Orbit(self.__get_current_config())

        self.view_3d, self.track_3d = self.__init_3d_view(blue_marble=False)
        self.view_2d, self.track_2d = self.__init_2d_view()

        for slider in self.config.values():
            slider.on_changed(self.__update_orbit)

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

        _, samples_points = self.orbit.get_samples(100)
        track_3d = view_3d.plot(*self.orbit(samples_points), lw=2)[0]

        return view_3d, track_3d


    def __init_2d_view(self) -> plt.Axes:

        view_2d = self.fig.add_subplot(self.__gs[0, 1], projection=Robinson())
        view_2d.coastlines(resolution='50m', lw=0.7, animated=False)
        view_2d.gridlines(animated=False)
        view_2d.set_global()

        sample_times, sample_points = self.orbit.get_samples(100)
        r, theta, phi = Orbit.to_spherical(self.orbit(sample_points))

        track_2d = view_2d.plot(phi.degree - 180, 
                                theta.degree, 
                                transform=Geodetic(),
                                c='g', lw=2)[0]

        return view_2d, track_2d


    def __init_config(self) -> dict:

        config = {}

        for k, param in enumerate([r"r_\mathrm{p}", "e", "i", r"\Omega", r"\omega"], 1):
            config[param] = Slider(self.fig.add_subplot(self.__gs[k, 0]), 
                                   f"${param}$", **PARAM_LIMITS[param])
        
        return config
    

    def __update_orbit(self, _) -> int:

        self.orbit = Orbit(self.__get_current_config())

        # TODO: calculate anomalies w.r.t. orbital speed
        sample_times, sample_points = self.orbit.get_samples(100)
        track_coordinates = self.orbit(sample_points)
        self.track_3d.set_data_3d(*track_coordinates)

        r, theta, phi = Orbit.to_spherical(self.orbit(sample_points))
        self.track_2d.set_data(phi.degree - 180, theta.degree)
        
        return 1


    def __get_current_config(self) -> dict:

        config = {}

        for key, slider in self.config.items():

            if key == r"r_\mathrm{p}":
                config['a'] = Orbit.calculate_semi_major_axis(
                    self.config[r"r_\mathrm{p}"].val + EARTH_RADIUS_KM, 
                    self.config['e'].val
                )
            elif key in [r"\Omega", r"\omega", "i"]:
                config[key] = slider.val * np.pi/180.
            else:
                config[key] = slider.val

        return config