pub trait Histogram {
    fn width(&self) -> usize;
    fn height_at(&self, horizontal_position: usize) -> i32;
}

pub mod histogram_chunked;
