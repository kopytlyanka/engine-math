pub mod vec2;
pub mod vec3;
pub mod vec4;

use crate::functions::constants::EPSILON;
use std::ops::Sub;
use std::option::Option::{None, Some};
use std::ops::{Div, Mul};

pub trait Vector
where
    Self: Sized,
    Self: Sub,
    Self: Div<f32>,
    Self: Mul<f32>,
    Self: Copy,
    Self: From<f32>,
    Self: From<<Self as Sub>::Output>,
    Self: From<<Self as Div<f32>>::Output>,
    Self: From<<Self as Mul<f32>>::Output>,
{
    fn zero() -> Self {
        Self::from(0.)
    }

    fn dim(self) -> usize;
    fn len(self) -> f32;
    fn dot(self, vector: Self) -> f32;

    fn try_normalize(self) -> Option<Self> {
        let len = self.len();
        if len < EPSILON {
            return None;
        }
        Some((self / len).into())
    }
    fn normalize(self) -> Self {
        self.try_normalize().expect("Can't normalize null vector")
    }

    fn try_angle(self, vector: Self) -> Option<f32> {
        let (self_len, vector_len) = (self.len(), vector.len());
        if self_len < EPSILON || vector_len < EPSILON {
            return None;
        }
        Some(
            self.dot(vector) / (self_len * vector_len)
                .clamp(-1., 1.)
                .acos()
        )
    }
    fn angle(self, vector: Self) -> f32 {
        self.try_angle(vector).expect(
            "It is not possible to calculate the angle between two vectors, one of which is null",
        )
    }

    fn is_orthogonal_to(self, vector: Self) -> bool {
        if self.dot(vector).abs() < EPSILON {
            return true;
        }
        false
    }

    fn try_reflect_with(self, vector: Self) -> Option<Self> {
        let norm = vector.try_normalize()?;
        Some((self - (norm * (2. * self.dot(vector))).into()).into())
    }

    fn reflect_with(self, vector: Self) -> Self {
        self.try_reflect_with(vector).expect(
            "It is not possible to reflect the vector relative to the null vector",
        )
    }
}
