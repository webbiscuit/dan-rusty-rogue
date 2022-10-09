use crate::components::point::Point;


#[derive(Debug, Copy, Clone)]
pub struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}
impl Rect {
    pub fn with_size(x: i32, y: i32, width: i32, height: i32) -> Rect {
        Rect {
            x,
            y,
            w: width,
            h: height,
        }
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }

    // Calls a function for each x/y point in the rectangle
    pub fn for_each_point<F>(&self, mut f: F)
    where
        F: FnMut(Point),
    {
        for y in self.y..self.y + self.h {
            for x in self.x..self.x + self.w {
                f(Point::new(x, y));
            }
        }
    }

    pub fn center(&self) -> Point {
        Point::new(self.x + (self.w / 2), self.y + (self.h / 2))
    }
}


