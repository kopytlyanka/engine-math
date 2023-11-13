pub mod mat2;
pub mod mat3;
pub mod mat4;
use mat2::Matrix2;
use mat3::Matrix3;
use mat4::Matrix4;

pub trait Matrix {
    type Row;
    type Column;

    fn scalar(value: f32) -> Self;
    fn idenity() -> Self
    where
        Self: Sized,
    {
        Self::scalar(1.)
    }
    fn zero() -> Self
    where
        Self: Sized,
    {
        Self::scalar(0.)
    }

    fn dim(self) -> usize;
    fn det(self) -> f32;

    fn get_row(self, i: usize) -> Self::Row;
    fn get_column(self, j: usize) -> Self::Column;
    fn transpose(self) -> Self;

    fn try_invert(self) -> Option<Self>
    where
        Self: Sized;
    fn invert(self) -> Self
    where
        Self: Clone,
    {
        self.try_invert()
            .expect("It is impossible to invert a singular matrix")
    }

    fn is_singular(self) -> bool
    where
        Self: Sized,
    {
        match self.try_invert() {
            Some(_) => false,
            None => true,
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod transform_matrix {
    use super::*;
    use crate::constants::PI;

    pub fn scaling_matrix_in_2d(a: f32, b: f32) -> Matrix2 {
        Matrix2::new([[a, 0.], [0., b]])
    }

    pub fn scaling_matrix_in_3d(a: f32, b: f32, c: f32) -> Matrix3 {
        Matrix3::new([[a, 0., 0.], [0., b, 0.], [0., 0., c]])
    }

    pub fn scaling_matrix_in_homogeneous_2d(a: f32, b: f32) -> Matrix3 {
        Matrix3::new([[a, 0., 0.], [0., b, 0.], [0., 0., 1.]])
    }

    pub fn scaling_matrix_in_homogeneous_3d(a: f32, b: f32, c: f32) -> Matrix4 {
        Matrix4::new([
            [a, 0., 0., 0.],
            [0., b, 0., 0.],
            [0., 0., c, 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn rotation_matrix_in_2d(phi: f32) -> Matrix2 {
        let c = phi.cos();
        let s = phi.sin();
        Matrix2::new([[c, -s], [s, c]])
    }

    pub fn rotation_matrix_in_3d_Ox(phi: f32) -> Matrix3 {
        let c = phi.cos();
        let s = phi.sin();
        Matrix3::new([[1., 0., 0.], [0., c, -s], [0., s, c]])
    }

    pub fn rotation_matrix_in_3d_Oy(psi: f32) -> Matrix3 {
        let c = psi.cos();
        let s = psi.sin();
        Matrix3::new([[c, 0., s], [0., 1., 0.], [-s, 0., c]])
    }

    pub fn rotation_matrix_in_3d_Oz(xi: f32) -> Matrix3 {
        let c = xi.cos();
        let s = xi.sin();
        Matrix3::new([[c, -s, 0.], [s, c, 0.], [0., 0., 1.]])
    }

    pub fn rotation_matrix_in_homogeneous_2d(phi: f32) -> Matrix3 {
        let c = phi.cos();
        let s = phi.sin();
        Matrix3::new([[c, -s, 0.], [s, c, 0.], [0., 0., 1.]])
    }

    pub fn rotation_matrix_in_homogeneous_3d_Ox(phi: f32) -> Matrix4 {
        let c = phi.cos();
        let s = phi.sin();
        Matrix4::new([
            [1., 0., 0., 0.],
            [0., c, -s, 0.],
            [0., s, c, 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn rotation_matrix_in_homogeneous_3d_Oy(psi: f32) -> Matrix4 {
        let c = psi.cos();
        let s = psi.sin();
        Matrix4::new([
            [c, 0., s, 0.],
            [0., 1., 0., 0.],
            [-s, 0., c, 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn rotation_matrix_in_homogeneous_3d_Oz(xi: f32) -> Matrix4 {
        let c = xi.cos();
        let s = xi.sin();
        Matrix4::new([
            [c, -s, 0., 0.],
            [s, c, 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn translate_matrix_in_homogeneous_2d(a: f32, b: f32) -> Matrix3 {
        Matrix3::new([[1., 0., a], [0., 1., b], [0., 0., 1.]])
    }

    pub fn translate_matrix_in_homogeneous_3d(a: f32, b: f32, c: f32) -> Matrix4 {
        Matrix4::new([
            [1., 0., 0., a],
            [0., 1., 0., b],
            [0., 0., 1., c],
            [0., 0., 0., 1.],
        ])
    }

    pub fn perspective_matrix_in_homogeneous_3d (
        z_far: f32,
        z_near: f32,
        aspect_ratio: f32,
        fov: f32,
    ) -> Matrix4 {
        let f = (PI * 0.5 - 0.5 * fov).tan();
        let range_inverse = 1. / (z_near - z_far);
        Matrix4::new([
            [f / (aspect_ratio), 0., 0., 0.],
            [0., f, 0., 0.],
            [0., 0., (z_near + z_far) * range_inverse, z_near * z_far * range_inverse * 2.],
            [0., 0., -1., 0.],
        ])
    }

    pub fn ortho_matrix_in_homogeneous_3d(left: f32, right: f32, bottom: f32, top: f32, near_val: f32, far_val: f32) -> Matrix4 {
        let rl_range = right - left;
        let tb_range = top - bottom;
        let val_range = far_val - near_val;
        let t_x = - (right + left) / rl_range;
        let t_y = - (top + bottom) / tb_range;
        let t_z = - (far_val + near_val) / val_range;
        Matrix4::new([
            [2. / rl_range, 0., 0., t_x],
            [0., 2. / tb_range, 0., t_y],
            [0., 0., -2. / val_range, t_z],
            [0., 0., 0., 1.],
        ])
    }

    pub fn lookat_matrix_in_homogeneous_3d() -> Matrix4 {
        todo!()
    }
}
