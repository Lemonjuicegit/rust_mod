pub struct Point {
    geo_type: String,
    x: f64,
    y: f64,
}

pub struct Line {
    geo_type: String,
    line_len: f64,
    xy: Vec<[f64; 2]>,
}
pub struct Polygon {
    geo_type: String,
    area: f64,
    centroid: Vec<Line>,
    bounds: [f64; 4],
}
impl Point {
    fn new() -> Self {
        Point {
            geo_type: "point".to_string(),
            x: 1.0,
            y: 1.0,
        }
    }
}
impl Line {
    pub fn new(xy_vec: Vec<[f64; 2]>) -> Self {
        Self {
            geo_type: "line".to_string(),
            line_len: 1.0,
            xy: xy_vec,
        }
    }
}
impl Polygon {
    fn new() {}
}


#[test]
fn test() {
    // let l = Line::new(vec![[1.0, 1.0]]);
    // println!("l:{:?}", l.xy);
}
