pub fn delete_and_backspace(s: &mut String) {
    let mut pud: Vec<char> = Vec::new();
    let mut go = 0;
    for c in s.chars(){
        if go > 0 && c != '+' {
            go -= 1;
            continue
        }else if c == '+' {
            go += 1;
            continue
        } else if c == '-'{
            pud.pop();
            continue
        }
        pud.push(c);
    }
      *s = String::new();
 
for c in pud{
    s.push(c);
}
}

pub fn do_operations(v: &mut [String]) {
    let mut result: Vec<String> = Vec::new();

    // READ the strings
    for c in v.iter() {
        let mut o1: isize = 0;
        let mut o2: isize = 0;
        let mut happened = false;
        let mut operator = ' ';

        for i in c.chars() {
            if i == '-' || i == '+' {
                operator = i;
                happened = true;
                continue;
            }

            let digit = (i as u32 - '0' as u32) as isize;

            if !happened {
                o1 = o1 * 10 + digit;
            } else {
                o2 = o2 * 10 + digit;
            }
        }

        let value = match operator {
            '+' => o1 + o2,
            '-' => o1 - o2,
            _ => 0,
        };

        result.push(value.to_string());
    }

    // WRITE back into caller's array
    for (i, s) in v.into_iter().enumerate() {
        *s = result[i].clone();
    }
}