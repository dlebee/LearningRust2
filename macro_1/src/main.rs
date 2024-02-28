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

macro_rules! print_value {
    ($val:expr) => {
        println!("The value is: {}", $val);
    };
}

macro_rules! sum {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($y:expr),+) => {
        $x + sum!($($y),+)
    };
}

macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => {
        std::cmp::min($x, find_min!($($y),+))
    };
}

macro_rules! find_max {
    ($x:expr) => {$x};
    ($x: expr, $($y: expr), +) => {
        std::cmp::max($x, find_max!($($y), +))
    }
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

    print_value!(david.get_name());
    println!("Sum result of 1, 2, 3, 4 is: {}", sum!(1, 2, 3, 4));


    let min = find_min!(10, 5, 20, 3, 15);
    println!("Minimum value is {}", min);

    let max = find_max!(10, 5, 20, 3, 15);
    println!("Maximum value is {}", max);

}