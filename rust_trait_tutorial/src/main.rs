// Sheep struc contains two fields on is name and other is naked
struct Sheep {naked: bool, name: &'static str}


trait Animal {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;
    // Method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise())
    }
        
    
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already namek...", self.name);
        }else {
            println!("{} gets a haircut!", self.name);
            self.naked = true
        }
    }

}

struct Cow {}

impl Animal for Cow {
    fn name(&self) -> &'static str {
        "COW"
    }
    fn new(name: &'static str) -> Self {
        Cow {  }
    }
    fn noise(&self) -> &'static str {
        "MOO"
    }
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise())
    }
}



// Implement the `Animal `trait for `   sheep`

impl Animal for Sheep {
    // `Self is the implementor type; `Sheep`
    fn new(name: &'static str) -> Sheep {
        Sheep { naked: false, name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaah"
        }else {
            "baaa"
        }
    }
    fn talk(&self) {
        println!("{} pause breifly ... {}", self.name, self.naked)
    }
    
   
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();

    let mut cow: Cow = Animal::new("Cow");
    cow.talk();
    

}
