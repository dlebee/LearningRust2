macro_rules! field_get_set {
    ($get_method_name: ident, $set_method_name: ident, $name:ident, $ty:ty) => {
        pub fn $get_method_name(&self) -> $ty {
            self.$name.clone()
        }

        pub fn $set_method_name(&mut self, value: $ty) {
            self.$name = value;
        }
    };
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    field_get_set!(get_name, set_name, name, String);
    field_get_set!(get_age, set_age, age, u32);
}

fn main() {
    let mut david = Person {
        name: String::from("David Lebee"),
        age: 34,
    };

    println!("Hello {}", david.get_name());

    david.set_name(String::from("David L."));
    println!("Updated name: {}", david.get_name());
}