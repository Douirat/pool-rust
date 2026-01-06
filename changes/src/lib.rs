#[derive(Debug, PartialEq, Eq, Clone)]
struct Light{
    pub alias: String,
    pub brightness: u8,
}
impl Light {
	 fn new(alias: &str) -> Self {
    Light{
    alias: String::from(alias),
	brightness:0,
    }
	}
    }
     fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    for light in lights {
        if light.alias == alias{
        light.brightness = value;
        }
    }
    }