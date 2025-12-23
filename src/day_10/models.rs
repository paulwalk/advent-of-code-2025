
#[derive(Debug, Clone,Eq, PartialEq)]
pub enum Light {
    On,
    Off,
}

impl Light {
    pub fn from_char(c: char) -> Light {
        match c {
            '#' => Light::On,
            '.' => Light::Off,
            _ => panic!("Invalid character: {}", c),
        }
    }
}

pub fn light_vector_from_chars(chars: Vec<char>) -> Vec<Light> {
    chars.into_iter().map(Light::from_char).collect()
}

