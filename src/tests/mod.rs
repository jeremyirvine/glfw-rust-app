use enum_iterator::Sequence;
use imgui_glfw_rs::imgui::Ui;

use crate::renderer::Renderer;

use self::{
    test_batch_rendering::TestBatchRendering, test_clear_color::TestClearColor,
    test_texture::TestTexture,
};

pub trait Testable: TestableID {
    fn render(&self, screen_size: (f32, f32), renderer: &Renderer);
    fn imgui_render(&mut self, screen_size: (f32, f32), ui: &Ui);
    fn update(&mut self, delta_time: f32);
}

pub trait TestableID {
    fn test_id() -> String
    where
        Self: Sized;
    fn test_name() -> String
    where
        Self: Sized;
}

#[derive(Debug, PartialEq, Sequence, Clone)]
pub enum TestType {
    ClearColor,
    Texture,
    BatchRendering,
}

impl From<TestType> for String {
    fn from(val: TestType) -> Self {
        match val {
            TestType::ClearColor => TestClearColor::test_name(),
            TestType::Texture => TestTexture::test_name(),
            TestType::BatchRendering => TestBatchRendering::test_name(),
        }
    }
}

pub enum TestTypeInternal {
    ClearColor(TestClearColor),
    Texture(TestTexture),
    BatchRendering(TestBatchRendering),
}

impl Testable for TestTypeInternal {
    fn render(&self, screen_size: (f32, f32), renderer: &Renderer) {
        match self {
            Self::Texture(t) => t.render(screen_size, renderer),
            Self::ClearColor(t) => t.render(screen_size, renderer),
            Self::BatchRendering(t) => t.render(screen_size, renderer),
        }
    }

    fn imgui_render(&mut self, screen_size: (f32, f32), ui: &Ui) {
        match self {
            Self::Texture(t) => t.imgui_render(screen_size, ui),
            Self::ClearColor(t) => t.imgui_render(screen_size, ui),
            Self::BatchRendering(t) => t.imgui_render(screen_size, ui),
        }
    }

    fn update(&mut self, delta_time: f32) {
        match self {
            Self::Texture(t) => t.update(delta_time),
            Self::ClearColor(t) => t.update(delta_time),
            Self::BatchRendering(t) => t.update(delta_time),
        }
    }
}

impl TestableID for TestTypeInternal {
    fn test_id() -> String {
        "__internal_type__".into()
    }

    fn test_name() -> String {
        "Internal Type (if you see this, something is seriously messed up)".into()
    }
}

impl TestTypeInternal {
    pub fn inner_test(&mut self) -> Box<&mut dyn Testable> {
        match self {
            TestTypeInternal::ClearColor(t) => Box::new(t),
            TestTypeInternal::Texture(t) => Box::new(t),
            TestTypeInternal::BatchRendering(t) => Box::new(t),
        }
    }

    pub fn test_name(&self) -> String {
        match self {
            TestTypeInternal::ClearColor(_) => TestClearColor::test_name(),
            TestTypeInternal::Texture(_) => TestTexture::test_name(),
            TestTypeInternal::BatchRendering(_) => TestBatchRendering::test_name(),
        }
    }

    pub fn test_id(&self) -> String {
        match self {
            TestTypeInternal::ClearColor(_) => TestClearColor::test_id(),
            TestTypeInternal::Texture(_) => TestTexture::test_id(),
            TestTypeInternal::BatchRendering(_) => TestBatchRendering::test_id(),
        }
    }
}

pub mod menu;
pub mod test_batch_rendering;
pub mod test_clear_color;
pub mod test_texture;
