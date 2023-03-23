pub trait GLComponent {
    fn renderer_id(&self) -> u32;
    fn bind(&self);
    fn unbind(&self);
}
