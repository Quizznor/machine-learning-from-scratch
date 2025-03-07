use delaunator::*;


pub struct Mesh {
    tri: Triangulation,
    pts: Vec<Point>
}

pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point   
}

impl Mesh {
    pub fn new(tri: Triangulation, pts: Vec<Point>) -> Mesh {
        Mesh { tri, pts }
    }

    pub fn contains_where(&self, point: &Point) -> Option<usize> {
        
        // First let's find the mesh node closest to the point. 
        // This should be a robust solution because the Delauney 
        // graph is the dual representation of the Voronoi graph.
        let (mut min_dist, mut min_index) = (f64::INFINITY, 0);
        for (index, mesh_node) in self.pts.iter().enumerate() {

            // how do I use the Geometry trait here? It's defined in utl.rs
            let (dx, dy) = (mesh_node.x - point.x, mesh_node.y - point.y);
            let this_distance = f64::sqrt(dx.powi(2) + dy.powi(2));
            if this_distance < min_dist {
                min_dist = this_distance;
                min_index = index;
            }
        }

        // Check the surrounding triangles that the closest point is a part of. 
        // If the connecting lines sum to 180 degrees we have found the triangle.
        for (i, triplet) in self.tri.triangles.chunks(3).into_iter().enumerate() {
            if triplet.contains(&min_index) {

                let (a1, b1) = (point.x - self.pts[triplet[0]].x, self.pts[triplet[0]].y - point.y);
                let (a2, b2) = (point.x - self.pts[triplet[1]].x, self.pts[triplet[1]].y - point.y);
                let (a3, b3) = (point.x - self.pts[triplet[2]].x, self.pts[triplet[2]].y - point.y);

                if a1 == 0. && b1 == 0. || a2 == 0. && b2 == 0. || a3 == 0. && b3 == 0. {return Some(i)}
                
                let phi1 = f64::atan( (a2*b1 - a1*b2) / (a1*a2 + b1*b2));
                let phi2 = f64::atan( (a3*b2 - a2*b3) / (a2*a3 + b2*b3));
                let phi3 = f64::atan( (a1*b3 - a3*b1) / (a3*a1 + b3*b1));

                println!("{} {} {} {} {} {}", a1, b1, a2, b2, a3, b3);
            }
        }
        None
    }

    pub fn get_triangulation(&self) -> &Triangulation {&self.tri}
    pub fn get_points(&self) -> &Vec<Point> {&self.pts}
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