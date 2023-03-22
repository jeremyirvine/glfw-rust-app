use std::ffi::c_void;

use glcall_macro::gl_call;
use stb_image::image::{LoadResult, Image};
use stb_image::stb_image::bindgen::{stbi_set_flip_vertically_on_load};

use crate::{gl_clear_errors, gl_log_errors};

enum TextureImage {
   U8(Image<u8>),
   F32(Image<f32>),
}

impl TextureImage {
    pub fn width(&self) -> usize {
        match self {
            TextureImage::F32(im) => im.width,
            TextureImage::U8(im) => im.width,
        }
    }

    pub fn height(&self) -> usize {
        match self {
            TextureImage::F32(im) => im.height,
            TextureImage::U8(im) => im.height,
        }
    }

    pub fn ptr(&self) -> *const c_void { 
        match self {
            TextureImage::F32(im) => im.data.as_ptr() as *const c_void,
            TextureImage::U8(im) => im.data.as_ptr() as *const c_void,
        }
    }
}

pub struct Texture {
    renderer_id: u32,
    image: TextureImage,
}

impl Drop for Texture {
    fn drop(&mut self) {
        gl_call!({ gl::DeleteTextures(1, &self.renderer_id); });
    }
}

impl Texture {
    pub fn new(path: String) -> Self {
        let mut instance = {
            unsafe { 
                stbi_set_flip_vertically_on_load(1);
            }
            let image = stb_image::image::load(path);

            let image = match image {
                LoadResult::ImageU8(image) => TextureImage::U8(image),
                LoadResult::ImageF32(image) => TextureImage::F32(image),
                LoadResult::Error(e) => panic!("Failed to load image: {}", e),
            };

            Self {
                renderer_id: 0,
                image,
            }
        };

        gl_call!({
            gl::GenTextures(1, &mut instance.renderer_id);
            gl::BindTexture(gl::TEXTURE_2D, instance.renderer_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D, 
                0, 
                gl::RGBA8 as i32, 
                instance.image.width() as i32, 
                instance.image.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                instance.image.ptr(),
            );
            instance.unbind();
        });

        instance
    }

    pub fn width(&self) -> usize {
        self.image.width()
    }
    pub fn height(&self) -> usize {
        self.image.height()
    }

    pub fn bind(&self, slot: u32) {
        gl_call!({ 
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_2D, self.renderer_id);
        });
    }

    pub fn unbind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0); }
    }
}
