pub fn find_marker(text: &str, window_length: usize) -> usize {
    let chars = text.chars().collect::<Vec<char>>();

    let mut i: usize = 0;
    let mut k: usize = 0;
    let mut found: bool; // first char is repeated in the the window of 4

    while i < chars.len() {
        found = false;

        let mut j = 1;
        while j < window_length - k {
            println!(
                "i={}\tj={}\tk={}\t{:#?}-{:#?}",
                i,
                j,
                k,
                chars[i + k],
                chars[i + k + j]
            );

            if chars[i + k] == chars[i + k + j] {
                found = true;
                i += 1;
                k = 0;
                break;
            }

            j += 1;
        }

        if !found {
            k += 1;

            if k == window_length - 1 {
                return i + window_length;
            };
        }
    }

    0
}
