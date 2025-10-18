from . import np

AXIS_3D_LIMIT_KM: int   = 30000     # view limit for 3D axis (km)
N_ORBITS: int           = 1         # number of orbits to draw

NEWTONS_CONSTANT: float = 6.674e-11 # grav. constant (m³ / kg s²)
EARTH_RADIUS_KM: int    = 6378      # radius of the earth (km)
EARTH_MASS_KG: float    = 5.972e24  # mass of the earth (kg)
EARTH_ROTATION: float   = 7.292e-5  # earth rotational velocity (rad/s)

PARAM_LIMITS = {
    r'r_\mathrm{p}': {"valmin": 0, 
                      "valmax": 36000, 
                      "valinit": 600, "valfmt": r"%.0f$\,$km"},
    'e': {"valmin": 0, "valmax": 1, "valinit": 0.737},
    'i': {"valmin": -90, "valmax": 90, "valinit": 63.4, "valfmt": r"%.0f$\,^\circ$"},
    r"\Omega": {"valmin": 0, "valmax": 360, "valinit": 270, "valfmt": r"%.0f$\,^\circ$"}, 
    r"\omega": {"valmin": 0, "valmax": 360, "valinit": 115, "valfmt": r"%.0f$\,^\circ$"},
}