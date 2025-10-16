use std::marker::PhantomData;
use crate::colour::{Colour8, BLANK};

pub struct FrameBuffer<T: FrameBufferTrait> {
    pub width_px: usize,
    pub height_px: usize,
    pub buf: T,
}

impl<T: FrameBufferTrait> FrameBuffer<T> {
    pub fn new(width_px: usize, height_px: usize, buf: T) -> Self {
        FrameBuffer {
            width_px,
            height_px,
            buf,
        }       
    }

    pub fn write_buf(&mut self, px_x:usize, px_y: usize, colour: &Colour8) -> Result<(), FrameBufError> {
        self.buf.write_buf(px_x, px_y, colour, self.width_px, self.height_px)
    }

    pub fn read_buf(&self, px_x:usize, px_y: usize) -> Result<Colour8, FrameBufError> {
        self.buf.read_buf(px_x, px_y, self.width_px, self.height_px)
    }

    pub fn clear_buf(&mut self) {
        for x in 0..self.width_px {
            for y in 0..self.height_px{
                let _ = self.buf.write_buf(x, y, &BLANK, self.width_px, self.height_px);
            }
        }
    }

    // Writes the contents of the frame buf to a scaled frame buf
    // This is inneficient
    // Especially because reading and writing to the frame buffer does a lot of coordinate and colour conversions
    pub fn scale(&self, scaled_frame_buf: &mut FrameBuffer<T>, scale_factor: usize) -> Result<(), FrameBufError>{
        for x_small in 0..self.width_px {
            for y_small in 0..self.height_px{
                let colour = self.read_buf(x_small, y_small)?;
                scaled_frame_buf.write_square(x_small * scale_factor, y_small * scale_factor, colour, scale_factor);
            }
        }

        Ok(())
    }

    // Writes a square with a solid colour to the frame buffer
    fn write_square(&mut self, px_x: usize, px_y: usize, colour: Colour8, size: usize) {
        for x in px_x..(px_x + size) {
            for y in px_y..(px_y + size) {
               let _ = self.write_buf(x, y, &colour);
            }   
        }
    }
}

pub enum FrameBufError {
    PixelOutsideBuf,
    Other,
}

pub trait FrameBufferTrait {
    // px_x and px_y are the pixels to write to
    // The origin of px_x and px_y is in the bottom left of the image

    // Write a colour to the buffer
    fn write_buf(&mut self, px_x: usize, px_y: usize, colour: &Colour8, width_px: usize, height_px: usize) -> Result<(), FrameBufError>;

    // Read a colour from the buffer
    fn read_buf(&self, px_x: usize, px_y: usize, width_px: usize, height_px: usize) -> Result<Colour8, FrameBufError>;
}