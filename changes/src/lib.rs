#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Light{
    pub alias: String,
    pub brightness: u8,
}
impl Light {
	pub fn new(alias: &str) -> Self {
    Light{
    alias: String::from(alias),
	brightness:0,
    }
	}
    }
    pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    for light in lights {
        if light.alias == alias{
        light.brightness = value;
        }
    }
    }