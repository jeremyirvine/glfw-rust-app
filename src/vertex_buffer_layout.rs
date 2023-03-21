use gl::types::{GLenum, GLuint, GLfloat, GLubyte};

pub fn size_of_type(gl_type: GLenum) -> u32 {
    match gl_type {
        gl::FLOAT => std::mem::size_of::<GLfloat>() as u32,
        gl::UNSIGNED_INT => std::mem::size_of::<GLuint>() as u32,
        gl::UNSIGNED_BYTE => std::mem::size_of::<GLubyte>() as u32,
        t => {
            panic!("Invalid size type: {}", t);
        }
    }
}

#[derive(Clone)]
pub struct VertexBufferElement {
    gl_type: GLenum,
    count: u32,
    normalized: bool,
}

impl VertexBufferElement {
    pub fn gl_type(&self) -> GLenum {
        self.gl_type
    }
    
    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn normalized(&self) -> bool {
        self.normalized
    }
}

#[derive(Default)]
pub struct VertexBufferLayout {
    stride: u32,
    elements: Vec<VertexBufferElement>,
}

impl VertexBufferLayout {
    pub fn stride(&self) -> u32 {
        self.stride
    }

    pub fn elements(&self) -> &Vec<VertexBufferElement> {
        &self.elements
    }

    pub fn with_floats(self, count: u32) -> Self {
        self.layout(gl::FLOAT, count, false)
    }

    pub fn with_uints(self, count: u32) -> Self {
        self.layout(gl::UNSIGNED_INT, count, false)
    }
    pub fn with_ubytes(self, count: u32) -> Self {
        self.layout(gl::UNSIGNED_BYTE, count, true)
    }

    fn layout(mut self, gl_type: GLenum, count: u32, normalized: bool) -> Self {
        let vbe = VertexBufferElement {
            gl_type,
            count,
            normalized,
        };

        self.elements.push(vbe);
        self.stride += count * size_of_type(gl_type);
        self
    }
}
