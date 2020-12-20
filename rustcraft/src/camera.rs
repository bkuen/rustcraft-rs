//! Types and traits representing various of cameras

use cgmath::{Vector3, Matrix4, Zero, InnerSpace, Point3, EuclideanSpace, Rad};
use std::ops::{Deref, DerefMut};

const WORLD_UP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

/// Camera
///
/// The basic structure of a camera
/// which could be moved and rotated
/// on all axis.
/// This implementation just calculates
/// the view matrix. The projection
/// matrix is specified in either an
/// orthographic or a perspective
/// camera.
pub struct Camera {
    /// The position of the camera
    pos: Vector3<f32>,
    /// The pitch of the camera
    pitch: f32,
    /// The yaw of the camera
    yaw: f32,
    /// The roll of the camera
    roll: f32,
    /// The vector which looks up of the camera
    up: Vector3<f32>,
    /// The vector which looks right of the camera
    right: Vector3<f32>,
    /// The vector which in which the camera looks
    look: Vector3<f32>,
    /// The view matrix of the camera
    view_matrix: Matrix4<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        let mut camera = Self {
            pos: Vector3::zero(),
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            up: Vector3::new(0f32, 1f32, 0f32),
            right: Vector3::zero(),
            look: Vector3::zero(),
            view_matrix: Matrix4::zero(),
        };
        camera.calc_view_matrix();
        camera
    }
}

impl Camera {
    /// Creates a new camera at the given location
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the camera
    pub fn at_pos(pos: Vector3<f32>) -> Self {
        let mut camera = Self {
            pos,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            up: Vector3::new(0f32, 1f32, 0f32),
            right: Vector3::zero(),
            look: Vector3::zero(),
            view_matrix: Matrix4::zero(),
        };
        camera.calc_view_matrix();
        camera
    }

    pub fn look_at(&mut self, look: Vector3<f32>) {
        self.look = look;
        self.calc_view_matrix();
    }

    /// Returns the position of the camera
    pub fn pos(&self) -> &Vector3<f32> {
        &self.pos
    }

    /// Returns the pitch of the camera
    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    /// Returns the yaw of the camera
    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    /// Returns the roll of the camera
    pub fn roll(&self) -> f32 {
        self.roll
    }

    /// Returns the look of the camera
    pub fn look(&self) -> Vector3<f32> {
        self.look
    }

    /// Returns the right of the camera
    pub fn right(&self) -> Vector3<f32> {
        self.right
    }

    /// Returns the up of the camera
    pub fn up(&self) -> Vector3<f32> {
        self.up
    }

    /// Returns the view matrix of the camera
    pub fn view_matrix(&self) -> &Matrix4<f32> {
        &self.view_matrix
    }

    /// Sets the position of the camera
    ///
    /// # Arguments
    ///
    /// * `pos` - The new position of the camera
    pub fn set_pos(&mut self, pos: Vector3<f32>) {
        self.pos = pos;
        self.calc_view_matrix();
    }

    /// Moves the camera with the given offset
    ///
    /// # Arguments
    ///
    /// * `offset` - An offset which should be passed to
    /// camera
    pub fn set_offset(&mut self, offset: Vector3<f32>) {
        self.pos += offset;
        self.calc_view_matrix();
    }

    /// Moves the camera along the forward plane (z-axis)
    ///
    /// # Arguments
    ///
    /// * `distance` - The distance the camera should be
    /// moved along the forward plane
    pub fn advance(&mut self, distance: f32) {
        self.pos += self.look * -distance;
        self.calc_view_matrix();
    }

    /// Moves the camera along the up plane (y-axis)
    ///
    /// # Arguments
    ///
    /// * `distance` - The distance the camera should be
    /// moved along the forward plane
    pub fn ascend(&mut self, distance: f32) {
        self.pos += self.up * distance;
        self.calc_view_matrix();
    }

    /// Moves the camera along the right plane (x-axis)
    ///
    /// # Arguments
    ///
    /// * `distance` - The distance the camera should be
    /// moved along the forward plane
    pub fn strafe(&mut self, distance: f32) {
        self.pos += self.right * distance;
        self.calc_view_matrix();
    }

    /// Rotates the camera by the given pitch, yaw and roll
    /// angles
    ///
    /// # Argument
    ///
    /// * `yaw` - The yaw angle by which the camera
    /// should be rotated.
    /// * `pitch` - The pitch angle by which the camera
    /// should be rotated.
    /// * `roll` - The roll angle by which the camera
    /// should be rotated.
    pub fn rotate(&mut self, yaw: f32, pitch: f32, roll: f32) {
        self.pitch += pitch.to_radians().clamp(
            -std::f32::consts::PI / 2.0 + 0.1,
             std::f32::consts::PI / 2.0 - 0.1,
        );
        self.yaw += yaw.to_radians();
        self.roll += roll.to_radians();

        self.look.x = self.pitch.cos() * self.yaw.sin();
        self.look.y = self.pitch.sin();
        self.look.z = self.pitch.cos() * self.yaw.cos();

        self.look = self.look.normalize();
        self.right = self.look.cross(WORLD_UP).normalize();
        self.up = self.right.cross(self.look).normalize();

        self.calc_view_matrix();
    }

    /// Rotates the camera by the given pitch angle.
    ///
    /// # Arguments
    ///
    /// * `angle` - The pitch angle by which the camera
    /// should be rotated.
    pub fn rotate_pitch(&mut self, angle: f32) {
        self.pitch += angle.to_radians();
        self.look = self.look * angle.to_radians().cos() + self.up * angle.to_radians().sin();
        self.look = self.look.normalize();
        self.up = self.look.cross(self.right);
        self.up *= -1.0;
        self.calc_view_matrix();
    }

    /// Rotates the camera by the given yaw angle.
    ///
    /// # Arguments
    ///
    /// * `angle` - The yaw angle by which the camera
    /// should be rotated.
    pub fn rotate_yaw(&mut self, angle: f32) {
        self.yaw += angle.to_radians();
        self.look = self.look * angle.to_radians().cos() + self.right * angle.to_radians().sin();
        self.look = self.look.normalize();
        self.right = self.look.cross(self.up);
        self.calc_view_matrix();
    }

    /// Rotates the camera by the given roll angle.
    ///
    /// # Arguments
    ///
    /// * `angle` - The roll angle by which the camera
    /// should be rotated.
    pub fn rotate_roll(&mut self, angle: f32) {
        self.roll += angle.to_radians();
        self.right = self.right * angle.to_radians().cos() + self.up * angle.to_radians().sin();
        self.right = self.look.normalize();
        self.up = self.look.cross(self.right);
        self.up *= -1.0;
        self.calc_view_matrix();
    }

    /// Calculates the view matrix of the camera
    pub fn calc_view_matrix(&mut self) {
        let target_pos = self.pos + self.look;
        self.view_matrix = Matrix4::look_at(Point3::from_vec(self.pos), Point3::from_vec(target_pos), self.up);
    }
}

/// Perspective Camera
///
/// The perspective camera is an advancement
/// of the basic camera and provides a projection
/// matrix simulating the real world (fov, far and
/// near plane)
pub struct PerspectiveCamera {
    /// The embedded basic camera
    camera: Camera,
    /// The fov of the camera
    fov: f32,
    /// The aspect ratio of the camera
    aspect_ratio: f32,
    /// The near plane of the camera
    near_plane: f32,
    /// The far plane of the camera
    far_plane: f32,
    /// The projection matrix of the camera
    proj_matrix: Matrix4<f32>,
}

impl Default for PerspectiveCamera {
    fn default() -> Self {
        let mut camera = Self {
            camera: Camera::default(),
            fov: 45.0,
            aspect_ratio: (180 / 720) as f32,
            near_plane: 0.1,
            far_plane: 100.0,
            proj_matrix: Matrix4::zero(),
        };
        camera.calc_view_matrix();
        camera
    }
}

impl Deref for PerspectiveCamera {
    type Target = Camera;

    fn deref(&self) -> &Self::Target {
        &self.camera
    }
}

impl DerefMut for PerspectiveCamera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.camera
    }
}

impl PerspectiveCamera {
    /// Creates a new camera at the given location.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the camera
    pub fn at_pos(pos: Vector3<f32>) -> Self {
        let mut camera = Self {
            camera: Camera::at_pos(pos),
            fov: 1.8,
            aspect_ratio: (1024 / 768) as f32,
            near_plane: 0.1,
            far_plane: 100.0,
            proj_matrix: Matrix4::zero(),
        };
        camera.calc_proj_matrix();
        camera
    }

    /// Returns the fov of the camera
    pub fn fov(&self) -> f32 {
        self.fov
    }

    /// Returns the aspect ratio of the camera
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    /// Returns the near plane of the camera
    pub fn near_plane(&self) -> f32 {
        self.near_plane
    }

    /// Returns the far plane of the camera
    pub fn far_plane(&self) -> f32 {
        self.far_plane
    }

    /// Sets the fov of the camera to a new value
    ///
    /// # Arguments
    ///
    /// * `fov` - The new fov value
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.calc_proj_matrix();
    }

    /// Sets the aspect ratio of the camera to a new value
    ///
    /// # Arguments
    ///
    /// * `aspect` - The new aspect ratio value
    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect_ratio = aspect;
        self.calc_proj_matrix();
    }

    /// Sets the near plane of the camera to a new value
    ///
    /// # Arguments
    ///
    /// * `near` - The new near plane value
    pub fn set_near_plane(&mut self, near: f32){
        self.near_plane = near;
        self.calc_proj_matrix();
    }

    /// Sets the far plane of the camera to a new value
    ///
    /// # Arguments
    ///
    /// * `far` - The new far plane value
    pub fn set_far_plane(&mut self, far: f32) {
        self.far_plane = far;
        self.calc_proj_matrix();
    }

    /// Returns the projection matrix of the camera
    pub fn proj_matrix(&self) -> &Matrix4<f32> {
        &self.proj_matrix
    }

    /// Calculates the projection matrix of the camera
    pub fn calc_proj_matrix(&mut self) {
        self.proj_matrix = cgmath::perspective(Rad(self.fov), self.aspect_ratio, self.near_plane, self.far_plane);
    }
}