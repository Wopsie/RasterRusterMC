use glam::{Mat4, Quat, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub const IDENTITY: Self = Self {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self 
    {
        Self {
            translation,
            rotation: rotation.normalize(),
            scale,
        }
    }

    pub fn local(&self) -> Mat4 {
        Mat4::from_translation(self.translation) 
            * Mat4::from_quat(self.rotation)
            * Mat4::from_scale(self.scale)
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn from_rotation(rotation: Quat) -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: rotation.normalize(),
            scale: Vec3::ONE,
        }
    }

    pub fn from_translation_rotation(translation: Vec3, rotation: Quat) -> Self {
        Self {
            translation,
            rotation: rotation.normalize(),
            scale: Vec3::ONE,
        }
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }
    
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * -Vec3::Z    //account for flipping of Z axis? 
    }

    // impl Into<Mat4> for Transform {  //prefered to implement From for Transform instead, but can be nice to explicitness
    //     fn into(self) -> Mat4 {
    //         self.local()
    //     }
    // }
}

pub enum TransformInitialParams {
    Identity,
    Translation(Vec3),
    Rotation(Quat),
    TranslationRotation(Vec3, Quat),
    // we could handle some fancy stuff like: FromMat4(Mat4),
}

impl From<TransformInitialParams> for Transform {
    fn from(params: TransformInitialParams) -> Self {
        match params {
            TransformInitialParams::Identity => Self::IDENTITY,
            TransformInitialParams::Translation(translation) => Self::from_translation(translation),
            TransformInitialParams::Rotation(rotation) => Self::from_rotation(rotation),
            TransformInitialParams::TranslationRotation(translation, rotation) => {
                Self::from_translation_rotation(translation, rotation)
            }
        }
    }
}