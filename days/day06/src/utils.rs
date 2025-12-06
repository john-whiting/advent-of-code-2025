fn get_at_ij<'a>(vec: &'a Vec<&str>, i: usize, j: usize) -> &'a u8 {
    vec.get(i)
        .unwrap_or(&"")
        .as_bytes()
        .get(j)
        .unwrap_or(&b'\0')
}

fn transpose_string_lines(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let lines_ref = &lines;

    (0..lines[0].len())
        .map(|j| {
            String::from_utf8(
                (0..lines.len())
                    .map(move |i| get_at_ij(lines_ref, i, j).clone())
                    .collect::<Vec<u8>>(),
            )
            .expect("string should be valid")
        })
        .collect::<Vec<String>>()
        .join("\n")
        .replace("\0", "")
}

fn reverse_string_lines(input: &str) -> String {
    input
        .lines()
        .map(|line| line.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn rotate_anticlockwise(input: &str) -> String {
    let reversed = reverse_string_lines(input);
    transpose_string_lines(&reversed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose_string() {
        let input = "123\n456\n789";
        let expected = "147\n258\n369";
        assert_eq!(transpose_string_lines(input), expected);

        let input = "123\n456";
        let expected = "14\n25\n36";
        assert_eq!(transpose_string_lines(input), expected);
    }

    #[test]
    fn test_reverse_string_lines() {
        let input = "abc\ndef";
        let expected = "cba\nfed";
        assert_eq!(reverse_string_lines(input), expected);
    }

    #[test]
    fn test_rotate_anticlockwise() {
        let input = "123\n456\n789";
        let expected = "369\n258\n147";
        assert_eq!(rotate_anticlockwise(input), expected);
    }
}
