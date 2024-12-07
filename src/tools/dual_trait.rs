use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

pub trait Algebra: Sized + Debug {
    type Item: Add<Output = Self::Item>
        + AddAssign
        + Sub<Output = Self::Item>
        + SubAssign
        + Mul<Output = Self::Item>
        + MulAssign
        + Div<Output = Self::Item>
        + DivAssign
        + Rem<Output = Self::Item>
        + RemAssign
        + Eq
        + Clone;
    fn new(first: Self::Item, last: Self::Item) -> Self;
    fn first(&self) -> Self::Item;
    fn last(&self) -> Self::Item;

    fn splat(value: Self::Item) -> Self {
        Self::new(value.clone(), value)
    }

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

    fn from_tuple(tuple: (Self::Item, Self::Item)) -> Self {
        Self::new(tuple.0, tuple.1)
    }

    fn tuple(&self) -> (Self::Item, Self::Item) {
        (self.first(), self.last())
    }

    fn into_dual<T: Algebra<Item = Self::Item>>(&self) -> T {
        T::new(self.first(), self.last())
    }

    fn add_self(&self) -> Self::Item {
        self.first() + self.last()
    }

    fn mul_self(&self) -> Self::Item {
        self.first() * self.last()
    }
}
