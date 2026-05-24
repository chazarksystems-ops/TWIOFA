use crate::cell::Material;

pub fn is_traversable(m: Material) -> bool {
    match m {
        Material::Air | Material::Tunnel | Material::Water => true,
        _ => false,
    }
}

pub fn is_diggable(m: Material) -> bool {
    match m {
        Material::Soil | Material::LooseSoil => true,
        _ => false,
    }
}

pub fn default_support(m: Material) -> u8 {
    match m {
        Material::Air | Material::Tunnel | Material::Water => 0,
        Material::Soil => 200,
        Material::LooseSoil => 60,
        Material::Carcass => 100,
        Material::Root | Material::Stone | Material::NestWall => 255,
    }
}

pub fn accepts_moisture(m: Material) -> bool {
    match m {
        Material::Air
        | Material::Tunnel
        | Material::Soil
        | Material::LooseSoil
        | Material::Water => true,
        _ => false,
    }
}
