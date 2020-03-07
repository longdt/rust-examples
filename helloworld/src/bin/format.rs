use std::fmt::{Display, Formatter, Result};

fn main() {
    for city in [City {
        name: "Dublin",
        lat: 53.347778,
        lon: -6.259722
    }, City {
        name: "Oslo",
        lat: 59.95,
        lon: 10.75
    }].iter() {
        println!("{}", city);
    }

    let color = Color {
        red: 100,
        green: 240,
        blue: 90
    };
    println!("{}", color);
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}", self.red, self.green, self.blue)
    }
}