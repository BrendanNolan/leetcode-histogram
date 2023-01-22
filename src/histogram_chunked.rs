use crate::Histogram;

fn calculate_bar_heights<H: Histogram>(histogram: &H) -> Vec<i32> {
    (0..histogram.width())
        .map(|horizontal_position| histogram.height_at(horizontal_position))
        .collect()
}

fn calculate_chunk_bounds<H: Histogram>(histogram: &H) -> Vec<i32> {
    let mut bounds = calculate_bar_heights(histogram);
    if !bounds.contains(&0) {
        bounds.push(0);
    }
    bounds.sort();
    bounds.dedup();
    bounds
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct VerticalChunk {
    pub bottom: i32,
    pub top: i32,
}

impl VerticalChunk {
    pub fn height(&self) -> i32 {
        self.top - self.bottom
    }
}

fn calculate_vertical_chunks<H: Histogram>(histogram: &H) -> Vec<VerticalChunk> {
    let chunk_bounds = calculate_chunk_bounds(histogram);
    chunk_bounds
        .iter()
        .zip(chunk_bounds.iter().skip(1))
        .map(|(bottom, top)| VerticalChunk {
            bottom: *bottom,
            top: *top,
        })
        .collect()
}

pub fn perform_chunked_computations<F, H: Histogram>(
    operation: &F,
    histogram: &H,
) -> Vec<(VerticalChunk, i32)>
where
    F: Fn(&H, i32) -> i32,
{
    calculate_vertical_chunks(histogram)
        .iter()
        .map(|chunk| (*chunk, operation(histogram, chunk.top) * chunk.height()))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::histogram_chunked::*;
    use crate::Histogram;

    impl Histogram for Vec<i32> {
        fn width(&self) -> usize {
            self.len()
        }

        fn height_at(&self, horizontal_position: usize) -> i32 {
            self[horizontal_position]
        }
    }

    fn count_filled_space(histogram: &Vec<i32>, height: i32) -> i32 {
        let mut filled_space_count = 0;
        for bar_height in histogram {
            if *bar_height >= height {
                filled_space_count += 1;
            }
        }
        filled_space_count
    }

    #[test]
    fn test_filled_space_count_0() {
        let histogram: Vec<i32> = vec![3, 1, 1];
        let chunked_results = perform_chunked_computations(&count_filled_space, &histogram);
        assert_eq!(
            chunked_results,
            vec![
                (VerticalChunk { bottom: 0, top: 1 }, 3),
                (VerticalChunk { bottom: 1, top: 3 }, 2)
            ]
        );
    }

    #[test]
    fn test_filled_space_count_1() {
        let histogram: Vec<i32> = vec![8, 3, 3, 0, 6, 3];
        let chunked_results = perform_chunked_computations(&count_filled_space, &histogram);
        assert_eq!(
            chunked_results,
            vec![
                (VerticalChunk { bottom: 0, top: 3 }, 15),
                (VerticalChunk { bottom: 3, top: 6 }, 6),
                (VerticalChunk { bottom: 6, top: 8 }, 2)
            ]
        );
    }

    #[test]
    fn test_calculate_vertical_chunks() {
        let histogram: Vec<i32> = vec![3, 1, 1];
        let chunks = calculate_vertical_chunks(&histogram);
        assert_eq!(
            chunks,
            vec![
                VerticalChunk { bottom: 0, top: 1 },
                VerticalChunk { bottom: 1, top: 3 }
            ]
        )
    }
}
