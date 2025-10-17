from . import np

class Orbit():

    def __init__(self, orbital_elements: dict) -> None:

        self.orbital_elements = self.__compute_missing(orbital_elements)

    
    def _get_ellipse(self) -> callable:

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

        rotation_matrix = (R_z(self.orbital_elements["omega"]) 
                           @ R_x(self.orbital_elements["i"]) 
                           @ R_z(self.orbital_elements["omega"]))
            
        x = lambda nu: radius(nu) * np.cos(nu)
        y = lambda nu: radius(nu) * np.sin(nu)
        z = lambda nu: np.zeros_like(nu)

        return lambda nu: rotation_matrix @ np.array([x(nu), y(nu), z(nu)])
    

    @staticmethod
    def __compute_missing(orbital_elements: dict) -> dict:
        return orbital_elements