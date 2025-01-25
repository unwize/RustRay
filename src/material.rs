use image::Rgb;

/// Struct that holds information about a primitive's visual properties.
/// Specular color and  shininess are both None for a solely diffuse material.
/// It is assumed that the ambient color of the material is equal to its diffuse color.
/// Material does not emit light.

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub diffuse_color: Rgb<u8>,
    pub specular_color: Option<Rgb<u8>>,
    pub shininess: Option<f32>,
    pub reflectivity: Option<f32>,
}

impl Material {
    pub fn new(diffuse_color: Rgb<u8>, specular_color: Option<Rgb<u8>>, shininess: Option<f32>, reflectivity: Option<f32>) -> Material {
        Self {
            diffuse_color,
            specular_color,
            shininess,
            reflectivity,
        }
    }

    /// Is true when the material is purely diffuse and possesses no reflectivity
    pub fn is_diffuse(&self) -> bool {
        self.specular_color.is_some() && self.shininess.is_none()
    }
}