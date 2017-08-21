use std::cmp::Ordering;

/// Represents a point in 2-dimensional space.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

/// Custom comparison for Points. Ordinality is determined by the x component first, then with the y component.
impl PartialOrd for Point2 {
    fn partial_cmp(&self, other: &Point2) -> Option<Ordering> {
        let x_cmp = self.x.partial_cmp(&other.x);
        match x_cmp {
            Some(Ordering::Equal) => self.y.partial_cmp(&other.y),
            _ => x_cmp,
        }
    }
}

impl Point2 {
    /// Returns a new Point2 instance.
    pub fn new(x: f64, y: f64) -> Point2 {
        return Point2 { x: x, y: y };
    }

    /// Returns the squared Euclidean distance between this point and another.
    pub fn dist_sq(&self, other: &Point2) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        return dx * dx + dy * dy;
    }
}

// for easy conversion between all possible tuples
impl<T: Into<f64> + Copy> From<(T, T)> for Point2 {
    fn from(coordinate: (T, T)) -> Point2 {
        Point2 {
            x: coordinate.0.into(),
            y: coordinate.1.into(),
        }
    }
}
impl<'a, T: Into<f64> + Copy> From<&'a (T, T)> for Point2 {
    fn from(coordinate: &'a (T, T)) -> Point2 {
        Point2 {
            x: coordinate.0.into(),
            y: coordinate.1.into(),
        }
    }
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Circle {
    pub center: Point2,
    pub radius: f64,
}

/// Returns the midpoint of two points along the line between them.
pub fn midpoint(a: Point2, b: Point2) -> Point2 {
    return Point2 {
        x: 0.5 * (a.x + b.x),
        y: 0.5 * (a.y + b.y),
    };
}

/// Returns the area of the parallelogram formed by three points.
pub fn area_of_parallelogram(a: &Point2, b: &Point2, c: &Point2) -> f64 {
    return (b.x - a.x) * (c.y - a.y) * (c.x - a.x) * (b.y - a.y);
}

/// Returns the area of the triangle formed by three points.
pub fn area_of_triangle(a: &Point2, b: &Point2, c: &Point2) -> f64 {
    return 0.5 * area_of_parallelogram(a, b, c);
}

/// Determines if the three points form a left-hand turn.
pub fn is_lht(a: &Point2, b: &Point2, c: &Point2) -> bool {
    return area_of_parallelogram(a, b, c) > 0.0;
}

/// Determines if the three points form a left-hand turn, or if they are colinear.
pub fn is_lht_or_on(a: &Point2, b: &Point2, c: &Point2) -> bool {
    return area_of_parallelogram(a, b, c) >= 0.0;
}

/// Determines if the three points form a right-hand turn.
pub fn is_rht(a: &Point2, b: &Point2, c: &Point2) -> bool {
    return area_of_parallelogram(a, b, c) < 0.0;
}

/// Determines if the three points form a right-hand turn, or if they are colinear.
pub fn is_rht_or_on(a: &Point2, b: &Point2, c: &Point2) -> bool {
    return area_of_parallelogram(a, b, c) <= 0.0;
}

/// Determines if the points C and D are on the same side of the line formed by points A and B.
pub fn is_same_side(a: &Point2, b: &Point2, c: &Point2, d: &Point2) -> bool {
    return is_lht(a, b, c) && is_lht(a, b, d) || is_rht(a, b, c) && is_rht(a, b, d);
}

/// Determines if the point D is contained within the circle formed by points A, B, and C.
pub fn in_circle(a: &Point2, b: &Point2, c: &Point2, d: &Point2) -> bool {
    let adx = a.x - d.x;
    let ady = a.y - d.y;
    let bdx = b.x - d.x;
    let bdy = b.y - d.y;
    let cdx = c.x - d.x;
    let cdy = c.y - d.y;

    let abdet = adx * bdy - bdx * ady;
    let bcdet = bdx * cdy - cdx * bdy;
    let cadet = cdx * ady - adx * cdy;
    let alift = adx * adx + ady * ady;
    let blift = bdx * bdx + bdy * bdy;
    let clift = cdx * cdx + cdy * cdy;

    return (alift * bcdet + blift * cadet + clift * abdet) > 0.0;
}