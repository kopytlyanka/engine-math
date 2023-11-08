pub mod functions;
pub mod matrices;
pub mod vectors;

use std::ops::Mul;

use matrices::*;
use mat2::Matrix2;
use mat3::Matrix3;
use mat4::Matrix4;
use transform_matrix::*;

use vectors::vec2::Vector2;
use vectors::vec3::Vector3;
use vectors::vec4::Vector4;

impl Mul<Vector2> for Matrix2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2::new(
            self[(0, 0)] * rhs.x + self[(0, 1)] * rhs.y,
            self[(1, 0)] * rhs.x + self[(1, 1)] * rhs.y,
        )
    }
}
impl Mul<Vector3> for Matrix3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(
            self[(0, 0)] * rhs.x + self[(0, 1)] * rhs.y + self[(0, 2)] * rhs.z,
            self[(1, 0)] * rhs.x + self[(1, 1)] * rhs.y + self[(1, 2)] * rhs.z,
            self[(2, 0)] * rhs.x + self[(2, 1)] * rhs.y + self[(2, 2)] * rhs.z,
        )
    }
}
impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        Vector4::new(
            self[(0, 0)] * rhs.x
                + self[(0, 1)] * rhs.y
                + self[(0, 2)] * rhs.z
                + self[(0, 3)] * rhs.w,
            self[(1, 0)] * rhs.x
                + self[(1, 1)] * rhs.y
                + self[(1, 2)] * rhs.z
                + self[(1, 3)] * rhs.w,
            self[(2, 0)] * rhs.x
                + self[(2, 1)] * rhs.y
                + self[(2, 2)] * rhs.z
                + self[(2, 3)] * rhs.w,
            self[(3, 0)] * rhs.x
                + self[(3, 1)] * rhs.y
                + self[(3, 2)] * rhs.z
                + self[(3, 3)] * rhs.w,
        )
    }
}

#[allow(dead_code)]
pub mod transform {

    use super::*;

    fn scale2(vector: Vector2, coefficients: (f32, f32)) -> Vector2 {
        let (a, b) = coefficients;
        scaling_matrix_in_2d(a, b) * vector
    }

    fn scale2x(vector: Vector2, coefficient: f32) -> Vector2 {
        scale2(vector, (coefficient, 1.))
    }

    fn scale2y(vector: Vector2, coefficient: f32) -> Vector2 {
        scale2(vector, (1., coefficient))
    }

    fn scale3(vector: Vector3, coefficients: (f32, f32, f32)) -> Vector3 {
        let (a, b, c) = coefficients;
        scaling_matrix_in_3d(a, b, c) * vector
    }

    fn scale3x(vector: Vector3, coefficient: f32) -> Vector3 {
        scale3(vector, (coefficient, 1., 1.))
    }

    fn scale3y(vector: Vector3, coefficient: f32) -> Vector3 {
        scale3(vector, (1., coefficient, 1.))
    }

    fn scale3z(vector: Vector3, coefficient: f32) -> Vector3 {
        scale3(vector, (1., 1., coefficient))
    }

    fn rotate2(vector: Vector2, phi: f32) -> Vector2 {
        rotation_matrix_in_2d(phi) * vector
    }

    fn rotate3x(vector: Vector3, phi: f32) -> Vector3 {
        rotation_matrix_in_3d_Ox(phi) * vector
    }

    fn rotate3y(vector: Vector3, psi: f32) -> Vector3 {
        rotation_matrix_in_3d_Oy(psi) * vector
    }

    fn rotate3z(vector: Vector3, xi: f32) -> Vector3 {
        rotation_matrix_in_3d_Oz(xi) * vector
    }

    pub mod homogeneous {

        use super::*;

        fn scale2(vector: Vector3, coefficients: (f32, f32)) -> Vector3 {
            let (a, b) = coefficients;
            scaling_matrix_in_homogeneous_2d(a, b) * vector
        }

        fn scale2x(vector: Vector3, coefficient: f32) -> Vector3 {
            scale2(vector, (coefficient, 1.))
        }

        fn scale2y(vector: Vector3, coefficient: f32) -> Vector3 {
            scale2(vector, (1., coefficient))
        }

        fn scale3(vector: Vector4, coefficients: (f32, f32, f32)) -> Vector4 {
            let (a, b, c) = coefficients;
            scaling_matrix_in_homogeneous_3d(a, b, c) * vector
        }

        fn scale3x(vector: Vector4, coefficient: f32) -> Vector4 {
            scale3(vector, (coefficient, 1., 1.))
        }

        fn scale3y(vector: Vector4, coefficient: f32) -> Vector4 {
            scale3(vector, (1., coefficient, 1.))
        }

        fn scale3z(vector: Vector4, coefficient: f32) -> Vector4 {
            scale3(vector, (1., 1., coefficient))
        }

        fn rotate2(vector: Vector3, phi: f32) -> Vector3 {
            rotation_matrix_in_homogeneous_2d(phi) * vector
        }

        fn rotate3x(vector: Vector4, phi: f32) -> Vector4 {
            rotation_matrix_in_homogeneous_3d_Ox(phi) * vector
        }

        fn rotate3y(vector: Vector4, psi: f32) -> Vector4 {
            rotation_matrix_in_homogeneous_3d_Oy(psi) * vector
        }

        fn rotate3z(vector: Vector4, xi: f32) -> Vector4 {
            rotation_matrix_in_homogeneous_3d_Oz(xi) * vector
        }

        fn shearing2(vector: Vector3, coefficients: (f32, f32)) -> Vector3 {
            let (a, b) = coefficients;
            shearing_matrix_in_homogeneous_2d(a, b) * vector
        }

        fn shearing2x(vector: Vector3, coefficient: f32) -> Vector3 {
            shearing2(vector, (coefficient, 1.))
        }

        fn shearing2y(vector: Vector3, coefficient: f32) -> Vector3 {
            shearing2(vector, (1., coefficient))
        }

        fn shearing3(vector: Vector4, coefficients: (f32, f32, f32)) -> Vector4 {
            let (a, b, c) = coefficients;
            shearing_matrix_in_homogeneous_3d(a, b, c) * vector
        }

        fn shearing3x(vector: Vector4, coefficient: f32) -> Vector4 {
            shearing3(vector, (coefficient, 1., 1.))
        }

        fn shearing3y(vector: Vector4, coefficient: f32) -> Vector4 {
            shearing3(vector, (1., coefficient, 1.))
        }

        fn shearing3z(vector: Vector4, coefficient: f32) -> Vector4 {
            shearing3(vector, (1., 1., coefficient))
        }
    }
}
