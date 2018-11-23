use nalgebra::{Vector2, Vector4};

pub trait Material {}

/// Represents a material with only colors.
pub struct ColoredMaterial {
    colors : Vec<Vector4<f32>>,
}

impl Material for ColoredMaterial {}

/// Represents a material with only a texture.
pub struct TexturedMaterial {
    texture_coords : Vec<Vector2<f32>>,
}

impl Material for TexturedMaterial {}