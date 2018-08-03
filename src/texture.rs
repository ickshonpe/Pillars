use gl;
use gl::types::*;
use image;
use std::io::{BufRead, Seek};

pub struct Texture {
    id: GLuint,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) }
    }
}

impl Texture {
    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn from_png<T: BufRead + Seek>(source: T) -> Texture {
        let image = image::load(source, image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let raw_image_data = image.into_raw();
        let mut texture_id: GLuint = 0; // dummy value
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                image_dimensions.0 as GLsizei,
                image_dimensions.1 as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                raw_image_data.as_ptr() as *const GLvoid,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Texture { id: texture_id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn release(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
