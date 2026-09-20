use super::{vertex, Vertex};
use super::opengl::{VertexArray, BufferUsage, Buffer, VertexBuffer, ElementBuffer, PrimitiveType};
use crate::error::{GameError, GameResult};
use glow::Context;
use std::rc::Rc;

pub struct Renderer {
    vertex_array: VertexArray,
    vertex_buffer: VertexBuffer,
    vertex_capacity: usize,
    element_buffer: ElementBuffer,
    element_capacity: usize,
}

impl Renderer {
    pub fn init_vertex_capacity(&mut self, usage: BufferUsage, capacity: usize) {
        self.vertex_buffer.bind();
        self.vertex_buffer.init_size(usage, vertex::ATTRIBUTE_STRIDE * capacity);
        init_vertex_attribute_pointer(&self.vertex_buffer);
        self.vertex_buffer.unbind();
        self.vertex_capacity = capacity;
    }

    pub fn init_with_vertices(&mut self, usage: BufferUsage, vertices: &[Vertex]) {
        self.vertex_buffer.bind();
        self.vertex_buffer.init_with_data(usage, &convert_vertices_to_data(vertices));
        init_vertex_attribute_pointer(&self.vertex_buffer);
        self.vertex_buffer.unbind();
        self.vertex_capacity = vertices.len();
    }

    pub fn update_vertices(&self, offset: usize, vertices: &[Vertex]) {
        self.vertex_buffer.bind();
        self.vertex_buffer.sub_data(offset, &convert_vertices_to_data(vertices));
        self.vertex_buffer.unbind();
    }

    pub fn vertex_capacity(&self) -> usize {
        self.vertex_capacity
    }

    pub fn init_element_capacity(&mut self, usage: BufferUsage, capacity: usize) {
        self.element_buffer.bind();
        self.element_buffer.init_size(usage, capacity);
        self.element_buffer.unbind();
        self.element_capacity = capacity;
    }

    pub fn init_with_elements(&mut self, usage: BufferUsage, elements: &[u16]) {
        self.element_buffer.bind();
        self.element_buffer.init_with_data(usage, elements);
        self.element_buffer.unbind();
        self.element_capacity = elements.len();
    }

    pub fn update_elements(&self, offset: usize, elements: &[u16]) {
        self.element_buffer.bind();
        self.element_buffer.sub_data(offset, elements);
        self.element_buffer.unbind();
    }

    pub fn element_capacity(&self) -> usize {
        self.element_capacity
    }

    pub fn draw_elements(&self, primitive: PrimitiveType, count: usize, offset: usize) {
        self.vertex_array.bind();
        self.vertex_array.draw_elements(primitive, count, offset);
        self.vertex_array.unbind();
    }
}

pub struct RendererBuilder {
    gl: Rc<Context>,
    vertex_array: VertexArray,
    vertex_buffer: Option<VertexBuffer>,
    vertex_capacity: Option<usize>,
    element_buffer: Option<ElementBuffer>,
    element_capacity: Option<usize>,
}

impl RendererBuilder {
    pub fn new(gl: Rc<Context>) -> GameResult<Self> {
        let vertex_array = VertexArray::new(gl.clone())
            .map_err(|error| GameError::InitError(error.into()))?;
        vertex_array.bind();
        Ok(Self {
            gl,
            vertex_array,
            vertex_buffer: None,
            vertex_capacity: None,
            element_buffer: None,
            element_capacity: None,
        })
    }

    fn assert_vertex_buffer_not_init(&self) {
        assert!(self.vertex_buffer.is_none(), "vertex buffer has been setup");
    }

    pub fn init_vertex_capacity(mut self, usage: BufferUsage, capacity: usize) -> Self {
        self.assert_vertex_buffer_not_init();
        let vertex_buffer = Buffer::new_vertex(self.gl.clone()).unwrap();
        vertex_buffer.bind();
        vertex_buffer.init_size(usage, vertex::ATTRIBUTE_STRIDE * capacity);
        init_vertex_attribute_pointer(&vertex_buffer);
        self.vertex_buffer = Some(vertex_buffer);
        self.vertex_capacity = Some(capacity);
        self
    }

    pub fn init_with_vertices(mut self, usage: BufferUsage, vertices: &[Vertex]) -> Self {
        self.assert_vertex_buffer_not_init();
        let vertex_buffer = Buffer::new_vertex(self.gl.clone()).unwrap();
        vertex_buffer.bind();
        vertex_buffer.init_with_data(usage, &convert_vertices_to_data(vertices));
        init_vertex_attribute_pointer(&vertex_buffer);
        self.vertex_buffer = Some(vertex_buffer);
        self.vertex_capacity = Some(vertices.len());
        self
    }

    fn assert_element_buffer_not_init(&self) {
        assert!(self.element_buffer.is_none(), "element buffer has been setup");
    }

    pub fn init_element_capacity(mut self, usage: BufferUsage, capacity: usize) -> Self {
        self.assert_element_buffer_not_init();
        let element_buffer = Buffer::new_element(self.gl.clone()).unwrap();
        element_buffer.bind();
        element_buffer.init_size(usage, capacity);
        self.element_buffer = Some(element_buffer);
        self.element_capacity = Some(capacity);
        self
    }

    pub fn init_with_elements(mut self, usage: BufferUsage, elements: &[u16]) -> Self {
        self.assert_element_buffer_not_init();
        let element_buffer = Buffer::new_element(self.gl.clone()).unwrap();
        element_buffer.bind();
        element_buffer.init_with_data(usage, elements);
        self.element_buffer = Some(element_buffer);
        self.element_capacity = Some(elements.len());
        self
    }

    pub fn build(self) -> GameResult<Renderer> {
        let vertex_array = self.vertex_array;
        let vertex_buffer = self.vertex_buffer
            .ok_or_else(|| GameError::InitError("must setup vertex buffer".into()))?;
        let vertex_capacity = self.vertex_capacity
            .ok_or_else(|| GameError::InitError("must setup vertex buffer".into()))?;
        let element_buffer = self.element_buffer
            .ok_or_else(|| GameError::InitError("must setup element buffer".into()))?;
        let element_capacity = self.element_capacity
            .ok_or_else(|| GameError::InitError("must setup element buffer".into()))?;
        vertex_array.unbind();
        vertex_buffer.unbind();
        element_buffer.unbind();
        Ok(Renderer {
            vertex_array,
            vertex_buffer,
            vertex_capacity,
            element_buffer,
            element_capacity,
        })
    }
}

fn init_vertex_attribute_pointer(vertex_buffer: &VertexBuffer) {
    vertex_buffer.set_attrib_pointer_f32(0, vertex::ATTRIBUTE_POSITION_SIZE, vertex::ATTRIBUTE_STRIDE, vertex::ATTRIBUTE_OFFSET_0);
    vertex_buffer.set_attrib_pointer_f32(1, vertex::ATTRIBUTE_UV_SIZE, vertex::ATTRIBUTE_STRIDE, vertex::ATTRIBUTE_OFFSET_1);
    vertex_buffer.set_attrib_pointer_f32(2, vertex::ATTRIBUTE_COLOR_SIZE, vertex::ATTRIBUTE_STRIDE, vertex::ATTRIBUTE_OFFSET_2);
}

fn convert_vertices_to_data(vertices: &[Vertex]) -> Vec<f32> {
    let mut data = Vec::with_capacity(vertex::ATTRIBUTE_STRIDE * vertices.len());
    for vertex in vertices {
        data.push(vertex.position.x);
        data.push(vertex.position.y);
        data.push(vertex.uv.x);
        data.push(vertex.uv.y);
        data.push(vertex.color.red);
        data.push(vertex.color.green);
        data.push(vertex.color.blue);
        data.push(vertex.color.alpha);
    }
    data
}
