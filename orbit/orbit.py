from matplotlib.gridspec import GridSpec
from cartopy.crs import Robinson, Geodetic
from astropy import coordinates, units
import matplotlib.pyplot as plt
from matplotlib import widgets
import scienceplots
import numpy as np

plt.style.use(['science'])

fig = plt.figure(figsize=[10, 7])
gs = GridSpec(5, 1, fig,
              height_ratios=[0.95, 0.025, 0.025, 0.025, 0.025])

MU = 5.972e24 * 6.6743e-11                      # in m^3 s^-2
OMEGA = 360 / (3600 * 24)                       # in deg / sec
T = lambda a: 2*np.pi * np.sqrt((a*1e3)**3/MU)  # in s

native = Geodetic()
proj = Robinson()
mouse_active, event_was_close = False, 0

# ax = fig.add_subplot(gs[0, 0])

# perigee, = ax.plot(0, 0, marker='o', c='b', ls='none', 
#                    label='Perigee')
# apogee, = ax.plot(180, 0, marker='o', ls='none',
#                   label='Apogee',
#                   markeredgecolor='b', markerfacecolor='w')
# orbit, = ax.plot(range(-180, 180), [0 for _ in range(360)], 
#                  c='b', zorder=0)

ax = fig.add_subplot(gs[0, 0], projection=proj)
ax.coastlines(resolution='50m', lw=0.7, animated=False)
ax.gridlines(animated=False)
ax.set_global()

perigee, = ax.plot(0, 0, marker='o', c='b', ls='none', 
                   transform=native, label='Perigee')
apogee, = ax.plot(180, 0, marker='o', ls='none',
                  transform=native, label='Apogee',
                  markeredgecolor='b', markerfacecolor='w')
orbit, = ax.plot(range(-180, 180), [0 for _ in range(360)], 
                 c='b', transform=native, zorder=0)

def click_was_close(event):

    if event.inaxes != ax: return

    event_x, event_y = native.transform_point(
            event.xdata,
            event.ydata,
            proj
        )
    
    global event_was_close
    dx = lambda vec: vec.get_xdata() - event_x
    dy = lambda vec: vec.get_ydata() - event_y

    if np.sqrt(dx(perigee)**2 + dy(perigee)**2) % 360 < 5: 
        event_was_close = 1
    elif np.sqrt(dx(apogee)**2 + dy(apogee)**2) % 360 < 5:
        event_was_close = 2

def button_press_callback(event):
    global mouse_active
    mouse_active = True
    click_was_close(event)

def button_move_callback(event):
    if not mouse_active \
        or not event_was_close \
        or event.inaxes is None:
        return
    else:
        event_coords = np.array(native.transform_point(
            event.xdata,
            event.ydata,
            proj
        ))

        match event_was_close:
            case 0: return  # nothing was clicked
            case 1:         # perigee was clicked
                perigee.set_data(*event_coords[:, np.newaxis])
            case 2:         # apogee was clicked
                apogee.set_data(*event_coords[:, np.newaxis])

def button_release_callback(_):
    global mouse_active
    global event_was_close
    mouse_active = False
    event_was_close = 0

    orbit_track = calculate_ellipse(...)
    orbit.set_data(*ground_track(orbit_track))
    fig.canvas.draw_idle()

fig.canvas.mpl_connect('button_press_event', button_press_callback)
fig.canvas.mpl_connect('motion_notify_event', button_move_callback)
fig.canvas.mpl_connect('button_release_event', button_release_callback)

eccentricity_slider = widgets.Slider(
    fig.add_subplot(gs[2, 0]), r"$e$", 0, 1, valinit=0)
periapsis_slider = widgets.Slider(
    fig.add_subplot(gs[1, 0]), r"$r_a$", 100, 36_000, 
    valinit=400, valfmt=r"%.1f km")
inclination_slider = widgets.Slider(
    fig.add_subplot(gs[3, 0]), r"$i$", -90, 90, 
    valinit=0, valfmt=r"%+.1f$\,^\circ$")
right_ascencion_slider = widgets.Slider(
    fig.add_subplot(gs[4, 0]), r"$\Omega_\mathrm{RA}$", -180, 180, 
    valinit=0, valfmt=r"%+.1f$\,^\circ$")

def calculate_ellipse(_):

    ecc = eccentricity_slider.val
    perigee_radius = periapsis_slider.val
    apogee_radius = perigee_radius * (1+ecc)/(1-ecc)
    
    a = 0.5 * (perigee_radius + apogee_radius)
    b = a * np.sqrt(1 - ecc**2)

    ap_theta, ap_phi = native.transform_point(*apogee.get_data(), proj)
    center = np.array(coordinates.spherical_to_cartesian(
        apogee_radius - perigee_radius, 
        ap_theta, 
        ap_phi))
    
    i = inclination_slider.val * np.pi/180
    w = right_ascencion_slider.val * np.pi/180

    if i == w == 0:
        u = np.array([1, 0, 0])
        v = np.array([0, 1, 0])
    else:
        si, ci = np.sin(i), np.cos(i)
        sw, cw = np.sin(w), np.cos(w)

        u = np.array([cw * ci, sw * ci, si])
        u /= np.linalg.norm(u)

        v = np.array([np.cos(w+np.pi/2), 
                      np.sin(w+np.pi/2), 
                      0])
        v /= np.linalg.norm(v)

    return lambda t: np.expand_dims(center, -1) \
        + a * np.outer(u, np.cos(t)) \
        + b * np.outer(v, np.sin(t))

def ground_track(track):

    r, theta, phi = coordinates.cartesian_to_spherical(
        *track(np.linspace(0, 2 * np.pi, 1000)))

    # something goes wrong here?
    x, y, z = proj.transform_points(native, 
                                    phi.degree - 180, 
                                    theta.degree).T
    
    x, y = phi.degree - 180, theta.degree

    return x, y

eccentricity_slider.on_changed(calculate_ellipse)
periapsis_slider.on_changed(calculate_ellipse)
inclination_slider.on_changed(calculate_ellipse)
right_ascencion_slider.on_changed(calculate_ellipse)

# ax.set_xlim(-180, 180)
# ax.set_ylim(-90, 90)

plt.tight_layout()
fig.legend(loc='outside upper center', ncol=2, fontsize=20)
plt.show()