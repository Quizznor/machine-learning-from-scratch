import matplotlib.pyplot as plt
from matplotlib import widgets
from matplotlib.gridspec import GridSpec
from cartopy.crs import Robinson, Geodetic
from astropy import units
import scienceplots
import numpy as np

plt.style.use(['science'])

fig = plt.figure(figsize=[10, 7])
gs = GridSpec(5, 1, fig,
              height_ratios=[0.95, 0.025, 0.025, 0.025, 0.025])

mouse_active, event_was_close = False, False
ax = fig.add_subplot(gs[0, 0], projection=Robinson())
ax.coastlines(resolution='50m', lw=0.7, animated=False)
ax.gridlines(animated=False)
ax.set_global()

apogee, = ax.plot(0, 0, marker='o', c='k', transform=Robinson())
orbit, = ax.plot([-180e5, 180e5], [0, 0], c='k', transform=Robinson())

def click_was_close(event):
    global event_was_close
    dx = apogee.get_xdata() - event.xdata
    dy = apogee.get_ydata() - event.ydata
    event_was_close = np.sqrt(dx**2 + dy**2) < 3e5

def button_press_callback(event):
    global mouse_active
    mouse_active = True
    click_was_close(event)

def button_release_callback(_):
    global mouse_active
    global event_was_close
    mouse_active = False
    event_was_close = False

def button_move_callback(event):
    if not mouse_active \
        or not event_was_close \
        or event.inaxes is None:
        return
    else:
        apogee.set_xdata([event.xdata])
        apogee.set_ydata([event.ydata])

        # update ellipse...
        latitudes, longitudes = get_ellipse()
        orbit.set_xdata(longitudes)
        orbit.set_ydata(latitudes)


        fig.canvas.draw_idle()
        # fig.canvas.flush_events()

fig.canvas.mpl_connect('button_press_event', button_press_callback)
fig.canvas.mpl_connect('button_release_event', button_release_callback)
fig.canvas.mpl_connect('motion_notify_event', button_move_callback)

eccentricity_slider = widgets.Slider(fig.add_subplot(gs[2, 0]), r"$e$", 0, 1, valinit=0)
periapsis_slider = widgets.Slider(fig.add_subplot(gs[1, 0]), r"$r_a$", 100, 36_000, valinit=400, valfmt=r"%.1f km")
inclination_slider = widgets.Slider(fig.add_subplot(gs[3, 0]), r"$i$", 0, 180, valinit=0, valfmt=r"%+.1f$\,^\circ$")
raan_slider = widgets.Slider(fig.add_subplot(gs[4, 0]), r"$\Omega_\mathrm{RA}$", 0, 360, valinit=0, valfmt=r"%+.1f$\,^\circ$")

# def get_widget_data():

#     return {
#         "alt": periapsis_slider.val << units.km,
#         "inc": inclination_slider.val << units.deg,
#         "raan": raan_slider.val << units.deg,
#         "ecc": eccentricity_slider.val << units.one,
#     }

def get_ellipse(**kwargs):

    apogee_lon = apogee.get_xdata()
    apogee_lat = apogee.get_ydata()

    latitudes = np.linspace(-180, 180, 360)
    longitudes = [apogee_lon for _ in range(360)]

    return latitudes, longitudes


# # def propagate(orbit):

# #     angular_velocity_earth = 360 / (3600 * 24)

# #     one_revolution = orbit.to_ephem(strategy=EpochBounds(
# #         min_epoch=0 << units.second, 
# #         max_epoch=orbit.period)
# #     )

# #     data = [
# #             coordinates.cartesian_to_spherical(
# #                 d.x, d.y, d.z)[1:] 
# #                 for d in one_revolution.sample()
# #             ]

# #     longitude, latitude = [], []
# #     print("NEW!!!")
# #     for d, t in zip(data, np.linspace(0., orbit.period.value, len(data), endpoint=True)):

# #         lon = (d[1] << units.deg).value # - angular_velocity_earth * t
# #         lat = (d[0] << units.deg).value

# #         print(f"{lon:.0f} {lat:.0f}")

# #         latitude.append(lat)
# #         longitude.append(lon)

# #     return latitude, longitude

# orbit, = ax.plot([], [], transform=Geodetic())

# def update_orbit(_):
#     # sattelite = Orbit.from_classical(Earth, )
#     # lats, lons = propagate(sattelite)


    
#     # args = get_widget_data()
#     # lons = np.linspace(0, 360, 360)
#     # lats = args[2] * np.cos((lons - 180) * np.pi/180)
#     lats, lons = propagate(Orbit.frozen(Earth, **get_widget_data()))

#     orbit.set_xdata(lons)
#     orbit.set_ydata(lats)
#     fig.canvas.draw_idle()


# eccentricity_slider.on_changed(update_orbit)
# periapsis_slider.on_changed(update_orbit)
# inclination_slider.on_changed(update_orbit)
# raan_slider.on_changed(update_orbit)

# update_orbit(...)
# plt.show()


plt.tight_layout()
plt.show()