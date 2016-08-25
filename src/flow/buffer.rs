pub struct Buffer {
    pub name: String
}

impl Buffer {
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
