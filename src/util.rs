use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

/// Creates a mesh from a triangle list geometry and its indices (used to determine facing
/// direction).
pub fn create_simple_mesh(geometry: Vec<[f32; 3]>, indices: Vec<u32>) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let points = geometry.len();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, geometry);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        (0..points).map(|_| [0.0, 0.0]).collect::<Vec<_>>(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        (0..points).map(|_| [0.0, 1.0, 0.0]).collect::<Vec<_>>(),
    );
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}
