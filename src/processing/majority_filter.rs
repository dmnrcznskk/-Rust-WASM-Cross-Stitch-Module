pub fn majority_filter(indices: &[u16], width: usize, height: usize) -> Vec<u16> {
    let mut result = indices.to_vec();

    for y in 0..height {
        for x in 0..width {
            let x_start = x.saturating_sub(1);
            let x_end = (x + 1).min(width - 1);
            let y_start = y.saturating_sub(1);
            let y_end = (y + 1).min(height - 1);

            let mut counts: Vec<(u16, u32)> = Vec::with_capacity(9);
            for ny in y_start..=y_end {
                for nx in x_start..=x_end {
                    let value = indices[ny * width + nx];
                    match counts.iter_mut().find(|(v, _)| *v == value) {
                        Some(entry) => entry.1 += 1,
                        None => counts.push((value, 1)),
                    }
                }
            }

            let own = indices[y * width + x];
            let max_count = counts.iter().map(|&(_, c)| c).max().unwrap_or(0);
            let own_count = counts.iter().find(|&&(v, _)| v == own).map_or(0, |&(_, c)| c);

            if own_count < max_count {
                if let Some(&(winner, _)) = counts.iter().find(|&&(_, c)| c == max_count) {
                    result[y * width + x] = winner;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lone_stitch_in_the_middle_is_replaced() {
        let input = vec![1, 1, 1, 1, 2, 1, 1, 1, 1];
        let output = majority_filter(&input, 3, 3);
        assert_eq!(output, vec![1; 9]);
    }

    #[test]
    fn uniform_image_stays_the_same() {
        let input = vec![5; 12];
        assert_eq!(majority_filter(&input, 4, 3), input);
    }

    #[test]
    fn tie_keeps_original_value() {
        let input = vec![1, 2];
        assert_eq!(majority_filter(&input, 2, 1), input);
    }

    #[test]
    fn no_new_values_appear() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let output = majority_filter(&input, 3, 3);
        assert!(output.iter().all(|v| input.contains(v)));
    }

    #[test]
    fn single_row_is_handled() {
        let input = vec![1, 1, 2, 1, 1];
        assert_eq!(majority_filter(&input, 5, 1), vec![1; 5]);
    }
}