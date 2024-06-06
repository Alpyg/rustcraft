#[allow(dead_code)]

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockType {
    id: String,
};

pub struct MeshData {
    pub pos: &str [[f32; 3]],
    pub uv: Vec<[f32; 2]>,
    pub color: &str [[f32; 4]],
    pub indices: &str [u32],
}

impl BlockType {
    pub fn get_texture_paths(&self) -> &'static str {
        "1.20.4/assets/minecraft/textures/block/"
    }
}
