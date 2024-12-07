use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

pub trait Algebra<
    T: Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
        + Rem<Output = T>
        + RemAssign
        + Eq,
>: Sized
{
    fn new(first: T, last: T) -> Self;
    fn first(&self) -> T;
    fn first_mut(&mut self) -> &mut T;
    fn last(&self) -> T;
    fn last_mut(&mut self) -> &mut T;

    fn add(&self, rhs: Self) -> Self {
        Self::new(self.first() + rhs.first(), self.last() + rhs.last())
    }

    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }

    fn sub(&self, rhs: Self) -> Self {
        Self::new(self.first() - rhs.first(), self.last() - rhs.last())
    }

    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs)
    }

    fn mul(&self, rhs: Self) -> Self {
        Self::new(self.first() * rhs.first(), self.last() * rhs.last())
    }

    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs)
    }

    fn div(&self, rhs: Self) -> Self {
        Self::new(self.first() / rhs.first(), self.last() / rhs.last())
    }

    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs)
    }

    fn rem(&self, rhs: Self) -> Self {
        Self::new(self.first() % rhs.first(), self.last() % rhs.last())
    }

    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs)
    }

    fn eq(&self, rhs: Self) -> bool {
        self.first() == rhs.first() && self.last() == rhs.last()
    }
}
