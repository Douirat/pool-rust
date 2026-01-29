pub fn twice<F>(f: F) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32 + Copy,
{
    move |x| f(f(x))
}
