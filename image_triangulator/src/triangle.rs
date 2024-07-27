use delaunator::Point;

pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point   
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Triangle {
        Triangle { p1, p2, p3}
    }
}

impl Triangle {
    pub fn contains(&self, point: Point) -> bool {

        if self.all_one_side(&point) {
            false
        } else {

            let (d1, d2, d3) = self.sign(&point);
            let has_neg = (d1 < 0.) || (d2 < 0.) || (d3 < 0.);
            let has_pos = (d1 > 0.) || (d2 > 0.) || (d3 > 0.);

            !(has_neg && has_pos)
        }
    }

    pub fn get_path(&self) -> Vec<(f64, f64)> {
        vec![
            (self.p1.x, self.p1.y),
            (self.p2.x, self.p2.y),
            (self.p3.x, self.p3.y),
            (self.p1.x, self.p1.y),
        ]
    }

    fn all_one_side(&self, point: &Point) -> bool {
        
        let (mut sum_x, mut sum_y): (i8, i8) = (0, 0);
        for p in [&self.p1, &self.p2, &self.p3].iter() {
            if p.x > point.x {
                sum_x += 1;
            } else {
                sum_x -= 1;
            }

            if p.y > point.y {
                sum_y += 1;
            } else {
                sum_y -= 1;
            }
        }

        sum_x.abs() == 3 || sum_y.abs() == 3
    }

    fn sign(&self, point: &Point) -> (f64, f64, f64) {

        let signlet = |p1: &Point, p2: &Point, p3: &Point| (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y) ;

        let d1 = signlet(point, &self.p1, &self.p2);
        let d2 = signlet(point, &self.p2, &self.p3);
        let d3 = signlet(point, &self.p3, &self.p1);

        (d1, d2, d3)
    }
}