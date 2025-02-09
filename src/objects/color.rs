pub struct Color {
    id: u32,
    name: String,
    hexadecimal: String,
}

impl Color {
    fn new(id: u32, name: String, hexadecimal: String) -> Self {
        Self {
            id,
            name,
            hexadecimal,
        }
    }
}
