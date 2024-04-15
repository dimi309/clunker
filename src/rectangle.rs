use crate::model::Model;

pub fn create_rectangle (
    topLeftX: f32,
    topLeftY: f32,
    topLeftZ: f32,
    bottomRightX: f32,
    bottomRightY: f32,
    bottomRightZ: f32) -> Model {

    let vertexData : [f32; 16] = [bottomRightX, bottomRightY, bottomRightZ, 1.0f32, 
    bottomRightX, topLeftY, topLeftZ, 1.0f32, topLeftX, topLeftY, topLeftZ, 1.0f32, 
    topLeftX, bottomRightY, bottomRightZ, 1.0f32];

    let indexData: [u32; 6] = [0, 1, 2, 2, 3, 0];
    let textureCoordsData: [f32; 8] = [1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    
    let m = Model {vertexData: vertexData.to_vec(), indexData: indexData.to_vec(), textureCoordsData: textureCoordsData.to_vec()};

    return m;

}
