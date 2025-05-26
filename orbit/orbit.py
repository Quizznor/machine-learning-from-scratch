from matplotlib.gridspec import GridSpec
from cartopy.crs import Robinson, Geodetic
import matplotlib.pyplot as plt
from matplotlib import widgets
from astropy import units
import scienceplots
import numpy as np

plt.style.use(['science'])

fig = plt.figure(figsize=[10, 7])
gs = GridSpec(5, 1, fig,
              height_ratios=[0.95, 0.025, 0.025, 0.025, 0.025])

native = Geodetic()
proj = Robinson()
mouse_active, event_was_close = False, 0
ax = fig.add_subplot(gs[0, 0], projection=proj)
ax.coastlines(resolution='50m', lw=0.7)
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
    else:
        print("asdasd")

def button_press_callback(event):
    global mouse_active
    mouse_active = True
    click_was_close(event)

def button_release_callback(_):
    global mouse_active
    global event_was_close
    mouse_active = False
    event_was_close = 0

def button_move_callback(event):
    if not mouse_active \
        or not event_was_close \
        or event.inaxes is None:
        return
    else:

        perigee_vector, apogee_vector, coords = get_ellipse(event)
        perigee.set_data(perigee_vector[1:, np.newaxis])
        apogee.set_data(apogee_vector[1:, np.newaxis])

        # orbit.set_data(*get_ellipse())

        fig.canvas.draw_idle()

fig.canvas.mpl_connect('button_press_event', button_press_callback)
fig.canvas.mpl_connect('button_release_event', button_release_callback)
fig.canvas.mpl_connect('motion_notify_event', button_move_callback)

eccentricity_slider = widgets.Slider(
    fig.add_subplot(gs[2, 0]), r"$e$", 0, 1, valinit=0)
periapsis_slider = widgets.Slider(
    fig.add_subplot(gs[1, 0]), r"$r_a$", 100, 36_000, 
    valinit=400, valfmt=r"%.1f km")
inclination_slider = widgets.Slider(
    fig.add_subplot(gs[3, 0]), r"$i$", 0, 180, 
    valinit=0, valfmt=r"%+.1f$\,^\circ$")
raan_slider = widgets.Slider(
    fig.add_subplot(gs[4, 0]), r"$\Omega_\mathrm{RA}$", 0, 360, 
    valinit=0, valfmt=r"%+.1f$\,^\circ$")

def get_ellipse(event):

    ecc = eccentricity_slider.val << units.one
    event_coords = native.transform_point(
        event.xdata,
        event.ydata,
        proj
    )

    match event_was_close:
        case 1:                             # perigee was clicked
            perigee_vector = np.array([
                r_p := periapsis_slider.val,
                *event_coords
                ])
            
            apogee_longitude = (perigee_vector[1] - 180) % 360
            if apogee_longitude > 180: apogee_longitude -= 360

            apogee_vector = np.array([
                r_a := r_p * (1+ecc)/(1-ecc),
                apogee_longitude,
                -perigee_vector[2]
                ])
        
        case 2:                             # apogee was clicked
            apogee_vector = np.array([
                r_a := periapsis_slider.val * (1+ecc)/(1-ecc),
                *event_coords
                ])
            
            perigee_longitude = (apogee_vector[1] - 180) % 360
            if perigee_longitude > 180: perigee_longitude -= 360
            
            perigee_vector = np.array([
                r_p := r_a * (1-ecc)/(1+ecc),
                perigee_longitude,
                -apogee_vector[2]
                ])

    # calculate ellipse...
    center = ...

    return perigee_vector, apogee_vector, [0]

plt.tight_layout()
fig.legend(loc='outside upper center', ncol=2, fontsize=20)
plt.show()