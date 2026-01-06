pub fn arrange_phrase(phrase: &str) -> String {
    let mut word_count = 0;
    let mut in_word = false;
    for c in phrase.chars() {
        if c != ' ' && !in_word {
            word_count += 1;
            in_word = true;
        } else if c == ' ' {
            in_word = false;
        }
    }
    let mut arranged: Vec<&str> = vec![""; word_count];

    let bytes = phrase.as_bytes();
    let mut start = 0;
    let mut i = 0;

    while i <= bytes.len() {
        if i == bytes.len() || bytes[i] == b' ' {
            if start != i {
                let word = &phrase[start..i];
                // Find the digit in the word
                for ch in word.chars() {
                    if ch.is_digit(10) {
                        let pos = ch.to_digit(10).unwrap() as usize - 1;
                        arranged[pos] = word;
                        break;
                    }
                }
            }
            start = i + 1;
        }
        i += 1;
    }

    let mut result = String::with_capacity(phrase.len());
    for (idx, &word) in arranged.iter().enumerate() {
        for ch in word.chars() {
            if !ch.is_digit(10) {
              result.push(ch);
            }
        }
        if idx != arranged.len() - 1 {
            result.push(' ');
        }
    }
    result
}
