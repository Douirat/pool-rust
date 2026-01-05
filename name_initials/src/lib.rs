pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for name in names{
    let mut sum: String = String::new();
   for (i, c) in name.split_whitespace().enumerate(){
        if i == 0{
        sum += &c[0..1];
        sum += &". ";
        } else {
        sum += &c[0..1];
        sum += &"."
        }
        
    }
    result.push(sum);
    }
    result
}