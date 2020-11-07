pub trait Ray {
    // TODO Remove some of these types
    type Point;
    type Vector;
    type Scalar;

    fn at(&self, t: Self::Scalar) -> Self::Point;

    fn origin(self) -> Self::Point;

    fn direction(self) -> Self::Vector;
}

// #[derive(Copy, Clone)]
// pub struct Ray {
//     pub origin: Point3,
//     pub direction: Point3,
// }
//
// impl Ray {
//     pub fn at(&self, t: Scalar) -> Point3 {
//         self.origin + (self.direction * t)
//     }
// }
