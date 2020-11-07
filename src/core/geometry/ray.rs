pub trait Ray {
    // TODO Remove some of these types
    type Point;
    type Vector;
    type Scalar;

    fn at(&self, t: Self::Scalar) -> Self::Point;

    fn origin(self) -> Self::Point;

    fn direction(self) -> Self::Vector;
}
