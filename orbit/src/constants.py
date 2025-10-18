from . import np

AXIS_3D_LIMIT_KM: int   = 30000     # view limit for 3D axis (km)

NEWTONS_CONSTANT: float = 6.674e-11 # grav. constant (m³ / kg s²)
EARTH_RADIUS_KM: int    = 6378      # radius of the earth (km)
EARTH_MASS_KG: float    = 5.972e24  # mass of the earth (kg)

PARAM_LIMITS = {
    r'r_\mathrm{p}': {"valmin": 0, 
                      "valmax": AXIS_3D_LIMIT_KM, 
                      "valinit": 400, "valfmt": r"%.0f$\,$km"},
    'e': {"valmin": 0, "valmax": 1, "valinit": 0},
    'i': {"valmin": -90, "valmax": 90, "valinit": 0, "valfmt": r"%.0f$\,^\circ$"},
    r"\Omega": {"valmin": 0, "valmax": 360, "valinit": 0, "valfmt": r"%.0f$\,^\circ$"}, 
    r"\omega": {"valmin": 0, "valmax": 360, "valinit": 0, "valfmt": r"%.0f$\,^\circ$"},
}