use priority_queue::PriorityQueue;
use ordered_float::OrderedFloat;
use std::f64::consts::PI;
use ndarray::Array;
use delaunator::*;

pub trait Geometry {
    fn distance(&self, other: &Self) -> f64;
    fn line(&self, other: &Self) -> Vec<Self> where Self: Sized;
    fn circle(&self, radius: f64) -> Vec<Self> where Self: Sized;
}

impl Geometry for Point {
    fn distance(&self, other: &Point) -> f64 {
        f64::sqrt( (self.x - other.x).powi(2) + (self.y - other.y).powi(2) )
    }

    fn line(&self, other: &Point) -> Vec<Point> {
        let mut line = vec![self.clone()];
        let (x_slope, y_slope) = (self.x - other.x, self.y - other.y);
        let d = self.distance(other);

        for increment in 0..d as i32 {
            let new_point = Point{
                x: (self.x - increment as f64 * (x_slope/d)).round(),
                y: (self.y - increment as f64 * (y_slope/d)).round()
            };

            if new_point != line[line.len()-1] {line.push(new_point);}
        }

        line
    }

    fn circle(&self, radius: f64) -> Vec<Point> {
        let mut circle = vec![Point{ x: self.x + radius, y: self.y}];

        for phi in Array::linspace(0., 2.* PI, 100) {
            let new_point = Point{ 
                x: (self.x + radius * f64::cos(phi)).round(),
                y: (self.y + radius * f64::sin(phi)).round()
            };
    
            if new_point != circle[circle.len()-1] {circle.push(new_point);}
        }
    
        circle
    }   
}

pub struct Mesh {
    tri: Triangulation,
    pts: Vec<Point>
}

impl Mesh {
    pub fn new(tri: Triangulation, pts: Vec<Point>) -> Mesh {
        Mesh { tri, pts }
    }

    pub fn contains_where(&self, point: Point) -> usize {

        // Sadly, sorting triangles by distance adds too much
        // overhead, and doesn't increase performance at all =(
        /*
        // Let's sort mesh by distance our target point, we will 
        // check the adjacent triangles via a priority queue query
        let mut priority_queue = PriorityQueue::new();
        for triplet in self.tri.triangles.chunks(3).into_iter() {

            let p1 = &self.pts[triplet[0]];
            let p2 = &self.pts[triplet[1]];
            let p3 = &self.pts[triplet[2]];

            // check if point lies in triangle bounding box
            let (mut sum_x, mut sum_y) = (0 as i32, 0 as i32);
            for p in [p1, p2, p3] {
                if p.x < point.x {sum_x -= 1}
                else if p.x > point.x {sum_x += 1}
                if p.y < point.y {sum_y -= 1}
                else if p.y > point.y {sum_y += 1}
            }

            if sum_x.abs() == 3 && sum_y.abs() == 3 {continue;}

            let d1 = OrderedFloat(point.distance(p1));
            let d2 = OrderedFloat(point.distance(p2));
            let d3 = OrderedFloat(point.distance(p3));
            priority_queue.push(triplet, (d1+d2+d3)/3.);
        }

        // let triangle_iter = priority_queue.into_sorted_iter()
        */
        let mut triangle_iter = self.tri.triangles.chunks(3);
        let mut triangle_index_counter = 0 as usize;
        loop {
            triangle_index_counter += 1;
            if let Some(t) = triangle_iter.next() {

                let (p1, p2, p3) = (&self.pts[t[0]], &self.pts[t[1]], &self.pts[t[2]]);
                if *p1 == point  || *p2 == point || *p3 == point {break triangle_index_counter-1;}
                
                // fast check if point is in bounding box of triangle
                let (mut sum_x, mut sum_y) = (0 as i8, 0 as i8);
                for p in [p1, p2, p3] {
                    if p.x < point.x {sum_x += 1}
                    else if p.x > point.x {sum_x -= 1}
                    if p.y < point.y {sum_y += 1}
                    else if p.y > point.y {sum_y -= 1}
                }
                
                if sum_x.abs() == 3 || sum_y.abs() == 3 {continue}
                
                // idk geometric shit, note that triangle needs to be oriented counter-clockwise for this to work
                // see blogpost over at http://totologic.blogspot.com/2014/01/accurate-point-in-triangle-test.html,
                // also https://stackoverflow.com/questions/2049582/how-to-determine-if-a-point-is-in-a-2d-triangle
                let (x, y) = (point.x, point.y);
                let (x1, y1) = (self.pts[t[0]].x, self.pts[t[0]].y);
                let (x2, y2) = (self.pts[t[1]].x, self.pts[t[1]].y);
                let (x3, y3) = (self.pts[t[2]].x, self.pts[t[2]].y);
                let a = ((y2 - y3)*(x - x3) + (x3 - x2)*(y - y3)) / ((y2 - y3)*(x1 - x3) + (x3 - x2)*(y1 - y3));
                let b = ((y3 - y1)*(x - x3) + (x1 - x3)*(y - y3)) / ((y2 - y3)*(x1 - x3) + (x3 - x2)*(y1 - y3));
                let c = 1. - a - b;
                
                if a < 0. || b < 0. || c < 0. {continue}
                if a > 1. || b > 1. || c > 1. {continue}
                
                break triangle_index_counter-1;
            }
            else {
                break 0
            }
        }
    }

    pub fn get_triangulation(&self) -> &Triangulation {&self.tri}
    
    pub fn get_points(&self) -> &Vec<Point> {&self.pts}

    pub fn get_size(&self) -> (usize, usize, usize) {
        (self.pts[0].x as usize, self.pts[0].y as usize, self.tri.triangles.len()/3)
    }
}