

pub fn create_rectangle(
    topLeftX: f32,
    topLeftY: f32,
    topLeftZ: f32,
    bottomRightX: f32,
    bottomRightY: f32,
    bottomRightZ: f32,
    vertexData: &mut [f32; 16],
    indexData: &mut [u32; 6],
    textureCoordsData: &mut [f32; 8],
) {
    vertexData[0] = bottomRightX;
    vertexData[1] = bottomRightY;
    vertexData[2] = bottomRightZ;
    vertexData[3] = 1.0f32;
    vertexData[4] = bottomRightX;
    vertexData[5] = topLeftY;
    vertexData[6] = topLeftZ;
    vertexData[7] = 1.0f32;
    vertexData[8] = topLeftX;
    vertexData[9] = topLeftY;
    vertexData[10] = topLeftZ;
    vertexData[11] = 1.0f32;
    vertexData[12] = topLeftX;
    vertexData[13] = bottomRightY;
    vertexData[14] = bottomRightZ;
    vertexData[15] = 1.0f32;

    indexData[0] = 0u32;
    indexData[1] = 1u32;
    indexData[2] = 2u32;
    indexData[3] = 2u32;
    indexData[4] = 3u32;
    indexData[5] = 0u32;

    textureCoordsData[0] = 1.0f32;
    textureCoordsData[1] = 1.0f32;
    textureCoordsData[2] = 1.0f32;
    textureCoordsData[3] = 0.0f32;
    textureCoordsData[4] = 0.0f32;
    textureCoordsData[5] = 0.0f32;
    textureCoordsData[6] = 0.0f32;
    textureCoordsData[7] = 1.0f32;
}
