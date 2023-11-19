use std::any::Any;
use std::sync::Arc;

pub trait MeshShared {
    fn vertex_list(&self) -> Vec<Arc<dyn Any + Send + Sync>>;
    fn index_list(&self) -> Vec<usize>;
}

pub struct Mesh<Vertex>
where
    Vertex: Clone + Send + Sync,
{
    vertices: Vec<Vertex>,
    indices: Vec<usize>,
}

impl<Vertex> Mesh<Vertex>
where
    Vertex: Clone + Send + Sync,
{
    pub fn new_with_data(vertices: Vec<Vertex>, indices: Vec<usize>) -> Self {
        Self { vertices, indices }
    }

    pub fn add_triangle(&mut self, v0: Vertex, v1: Vertex, v2: Vertex) {
        let idx = self.vertices.len() as usize;
        self.vertices.push(v0);
        self.vertices.push(v1);
        self.vertices.push(v2);
        self.indices.push(idx);
        self.indices.push(idx + 1);
        self.indices.push(idx + 2);
    }

    pub fn add_mesh(&mut self, mesh: &Mesh<Vertex>) {
        let idx = self.vertices.len() as usize;
        for vertex in &mesh.vertices {
            self.vertices.push(vertex.clone());
        }
        for index in &mesh.indices {
            self.indices.push(idx + index);
        }
    }
}

impl<Vertex> MeshShared for Mesh<Vertex>
where
    Vertex: Clone + Send + Sync + 'static,
{
    fn vertex_list(&self) -> Vec<Arc<dyn Any + Send + Sync>> {
        self.vertices
            .iter()
            .map(|vertex| Arc::new(vertex.clone()) as Arc<dyn Any + Send + Sync>)
            .collect::<Vec<Arc<dyn Any + Send + Sync>>>()
    }

    fn index_list(&self) -> Vec<usize> {
        self.indices.clone()
    }
}
