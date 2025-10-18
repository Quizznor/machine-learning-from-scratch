from typing import Tuple
from functools import cache
from astropy import coordinates

from . import np
from .constants import *

class Orbit():

    mu = NEWTONS_CONSTANT * EARTH_MASS_KG

    def __init__(self, orbital_elements: dict) -> None:

        self.orbital_elements = orbital_elements
        self.track = self.__get_ellipse()


    def __call__(self, nu: np.ndarray) -> np.ndarray:
        return self.track(nu)

    
    def __get_ellipse(self) -> callable:

        num = self.orbital_elements["a"] * (1 - self.orbital_elements["e"]**2)
        denum = lambda nu: 1 + self.orbital_elements["e"] * np.cos(nu)
        radius = lambda nu: num / denum(nu)

        R_z = lambda w: np.array([
            [np.cos(w), -np.sin(w), 0],
            [np.sin(w),  np.cos(w), 0],
            [         0,         0, 1],
        ])

        R_x = lambda w: np.array([
            [1,         0,          0],
            [0, np.cos(w), -np.sin(w)],
            [0, np.sin(w),  np.cos(w)],
        ])

        rotation_matrix = (R_z(self.orbital_elements[r"\Omega"]) 
                           @ R_x(self.orbital_elements["i"]) 
                           @ R_z(self.orbital_elements[r"\omega"]))
            
        # orbit in the perifocal plane
        x = lambda nu: radius(nu) * np.cos(nu)
        y = lambda nu: radius(nu) * np.sin(nu)
        z = lambda nu: np.zeros_like(nu)

        # rotate to principle axes
        return lambda nu: rotation_matrix @ np.array([x(nu), y(nu), z(nu)])
    

    @staticmethod
    def calculate_semi_major_axis(perigee: float, eccentricity: float) -> float:
        """calculate the size of the semi major axis of an elliptical orbit in km"""

        apogee = perigee * (1+eccentricity)/(1-eccentricity)
        return (apogee + perigee) / 2
    

    @staticmethod
    def calculate_orbital_period(semi_major_axis: float) -> float:
        """calculate the orbital period of an elliptical orbit in seconds"""

        return 2 * np.pi * np.sqrt(semi_major_axis**3 / Orbit.mu)
    

    @cache
    def get_samples(self, n_samples_per_orbit: int = 100) -> Tuple[np.ndarray]:
        """get uniformly distributed time (seconds) and corresponding true anomaly"""

        e = self.orbital_elements["e"]
        M, E = [np.linspace(0, N_ORBITS*2*np.pi, N_ORBITS*n_samples_per_orbit) for _ in range(2)]

        # numerically solve for true anomaly 
        # max. 10 iterations for Newton method
        for _ in range(10):
            num = E - e*np.sin(E) - M
            denum = 1 - e*np.cos(E)
            E -= (dE := num/denum)

            if np.all(np.abs(dE) < 1e-3):
                break

        return M / np.sqrt(self.mu/(self.orbital_elements["a"]*1e3)**3), E


    @staticmethod
    def to_spherical(cartesian: np.ndarray) -> np.ndarray:
        return coordinates.cartesian_to_spherical(*cartesian)
    

    @staticmethod
    def add_rotation(seconds: np.ndarray, lon: np.ndarray) -> np.ndarray:
        print(f"len orbit = {seconds[-1]/3600:.2f} h")
        return lon - 180/np.pi * seconds * EARTH_ROTATION