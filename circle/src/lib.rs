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
for c in v {
    let mut o1: isize = 0;
    let mut o2: isize  = 0;
    let mut happened: bool = false;
    let mut operator = String::new();
    for i in c.chars(){
    if i == '-' || i == '+' || i == '*' {
        operator.push(i);
        happened = true;
        continue
    }
    if !happened{
            o1 = (o1 * 10) + (((i as u32 ) as isize) - 48);
        
    } else {
        o2  = (o2 * 10) + (((i as u32 ) as isize) - 48);
      
    }
    
    }
    if operator == "+" {
        println!("{}", (o1 + o2));
    }
    if operator == "-" {
          println!("{}", (o1 - o2));
    }
}
}