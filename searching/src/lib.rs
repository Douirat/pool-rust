pub fn search(array: &[i32], key: i32) -> Option<usize> {
   for (i, d) in array.into_iter().enumerate().rev(){
    if *d == key{
        return Some(i)
    }
   }
   None
}
