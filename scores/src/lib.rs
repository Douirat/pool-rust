use std::collections::HashMap;

pub fn score(s: &str) -> u64 {
    let mut m:HashMap<char, u64> = HashMap::new();

    let mut total:u64 = 0;
    for c in 'a'..='z'{
        
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l'| 'n'| 'r' | 's' | 't'=>  m.insert(c, 1),
             'd' | 'g' =>  m.insert(c, 2),
               'b' | 'c' =>  m.insert(c, 3),
                'm' | 'p' =>  m.insert(c, 3),
                 'f' | 'h' =>  m.insert(c, 4),
                  'v' | 'w' |'y' =>  m.insert(c, 4),
                   'k' =>  m.insert(c, 5),
                    'j' |'x' =>  m.insert(c, 8),
                     'q' | 'z' =>  m.insert(c, 10),
                       _ => continue,
        };
    }

    for c in 'A'..='Z'{
             match c {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L'| 'N'| 'R' | 'S' | 'T'=>  m.insert(c, 1),
             'D' | 'G' =>  m.insert(c, 2),
               'B' | 'C' =>  m.insert(c, 3),
                'M' | 'P' =>  m.insert(c, 3),
                 'F' | 'H' =>  m.insert(c, 4),
                  'V' | 'W' | 'Y' =>  m.insert(c, 4),
                   'k' =>  m.insert(c, 5),
                    'J' |'X' =>  m.insert(c, 8),
                     'Q' | 'Z' =>  m.insert(c, 10),
                     _ => continue,
        };
    }

    for c in s.chars() {
        let v = match m.get(&c) {
            Some(n) => n,
            None => &0,
        };
        total += v;
    }
    total
}