extern crate nalgebra_glm as glm;

enum ValueType {
    number = 0,
    charstring,
    character,
    MARKER,
}

struct Token {
    valueType: ValueType,
    value: String,
    next: Box<Token>,
    name: String,
}

struct Node {
    index: u32,
    transformation: glm::Mat4x4,
    rotation: glm::Quat,
    scale: glm::Vec3,
    translation: glm::Vec3,
    skin: u32,
    noSkin: bool,
    mesh: u32,
    children: Vec<Box<Node>>,
}

pub struct GlbFile {
    rootToken: Box<Token>,

}
