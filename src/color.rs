pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn into_vec(self) -> Vec<u8> {
        vec![self.r, self.g, self.b]
    }

    pub fn into_array(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}