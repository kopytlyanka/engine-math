pub mod vec2;
pub mod vec3;
pub mod vec4;

use crate::functions::constants::EPSILON;
use std::option::Option::{None, Some};
use std::{
    fmt::Display,
    ops::{Div, Mul},
};

pub trait Vector {
    fn zero() -> Self
    where
        Self: From<f32>,
    {
        Self::from(0.)
    }

    fn dim(self) -> usize;
    fn len(self) -> f32;

    fn try_normalize(self) -> Option<Self>
    where
        Self: Sized,
        Self: Display,
        Self: Div<f32>,
        Self: Copy,
        Self: From<<Self as Div<f32>>::Output>,
    {
        let len = self.len();
        if len < EPSILON {
            return None;
        }
        Some((self.clone() / len).into())
    }
    fn normalize(self) -> Self
    where
        Self: Sized,
        Self: Display,
        Self: Div<f32>,
        Self: Copy,
        Self: From<<Self as Div<f32>>::Output>,
    {
        self.try_normalize().expect("Can't normalize null vector")
    }

    fn try_angle(self, vector: Self) -> Option<f32>
    where
        Self: Sized,
        Self: Display,
        Self: Copy,
        Self: Mul,
        f32: From<<Self as Mul>::Output>,
    {
        let (self_len, vector_len) = (self.len(), vector.len());
        if self_len < EPSILON {
            return None;
        }
        if vector_len < EPSILON {
            return None;
        }
        Some(
            (f32::from(self.clone() * vector) / (self_len * vector_len))
                .clamp(-1., 1.)
                .acos(),
        )
    }
    fn angle(self, vector: Self) -> f32
    where
        Self: Sized,
        Self: Display,
        Self: Copy,
        Self: Mul,
        f32: From<<Self as Mul>::Output>,
    {
        self.try_angle(vector).expect(
            "It is not possible to calculate the angle between two vectors, one of which is null",
        )
    }

    fn is_orthogonal_to(self, vector: Self) -> bool
    where
        Self: Sized,
        Self: Mul,
        f32: From<<Self as Mul>::Output>,
    {
        if f32::from(self * vector).abs() < EPSILON {
            return true;
        }
        false
    }
}
