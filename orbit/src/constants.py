from . import np

EARTH_RADIUS_KM: int    = 6378      # radius of the earth (km)
AXIS_3D_LIMIT_KM: int   = 15000     # view limit for 3D axis (km)

PARAM_LIMITS = {
    'a': {"valmin": 1.2*EARTH_RADIUS_KM, 
          "valmax": AXIS_3D_LIMIT_KM, 
          "valinit": 1.2*EARTH_RADIUS_KM},

    'e': {"valmin": 0, "valmax": 1, "valinit": 0},
    'i': {"valmin": 0, "valmax": 1, "valinit": 0},
    'i': {"valmin": 0, "valmax": np.pi/2, "valinit": 0},
    r"\Omega": {"valmin": 0, "valmax": np.pi, "valinit": 0}, 
    r"\omega": {"valmin": 0, "valmax": np.pi, "valinit": 0},
}