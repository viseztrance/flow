pub struct Buffer {
    pub name: String
}

impl Buffer {
    pub fn from_names(names: Vec<&str>) -> Vec<Buffer> {
        names
            .iter()
            .map(|name| Buffer::new(name.to_string()))
            .collect()
    }

    pub fn new(name: String) -> Buffer {
        Buffer {
            name: name
        }
    }

    pub fn render(&self, lines: &Vec<String>) {
        println!("Printing {}:", self.name);
        println!("------------------------------------");
        for line in lines {
            println!("{}", line);
        }
    }
}
