use crate::core::color::Color;
use crate::core::geometry::{Point3, Vec2, Vec3};
use crate::core::math::rand::{rand, rand_between};
use crate::textures::Texture;

// TODO Set as parameter of Perlin?
const POINTS: usize = 256;

pub struct Perlin {
    ran_float: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let ran_float: Vec<f64> = (0..POINTS).map(|_| -> f64 { rand() }).collect();

        let perm_x = Self::generate_random_perm();
        let perm_y = Self::generate_random_perm();
        let perm_z = Self::generate_random_perm();

        Self {
            ran_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    #[allow(clippy::cast_sign_loss)]
    pub fn noise(&self, point: Point3) -> f64 {
        let i = ((4.0 * point.x()).abs() as usize) & 255;
        let j = ((4.0 * point.y()).abs() as usize) & 255;
        let k = ((4.0 * point.z()).abs() as usize) & 255;

        self.ran_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_y[k]]
    }

    fn generate_random_perm() -> Vec<usize> {
        let p: Vec<usize> = (0..POINTS).collect();

        Self::permutated(p.as_slice())
    }

    fn permutated(vec: &[usize]) -> Vec<usize> {
        let mut copy = vec.to_vec();

        let len = vec.len();

        for i in 0..len {
            let target = rand_between(i, len);

            copy.swap(i, target);
        }

        copy
    }
}

pub struct PerlinTexture {
    noise: Perlin,
}

impl PerlinTexture {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
        }
    }
}

impl Texture for PerlinTexture {
    fn value(&self, _coord: Vec2, point: Vec3) -> Vec3 {
        Color(1.0, 1.0, 1.0) * self.noise.noise(point)
    }
}
