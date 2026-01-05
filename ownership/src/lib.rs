pub fn first_subword(mut s:  String) -> String {
for (i, c) in s.chars().enumerate(){
    if i == 0 {
        continue
    }
    if c == '_' || c.is_uppercase() {
        s.truncate(i);
        break
    }
}
s
}