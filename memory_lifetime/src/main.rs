fn main() {
    needs_help();
    ok();
}

fn needs_help() {
    let value_one = 200;
    let value_two = 300;
    let value = explicit_lifetime(&value_one, &value_two);
    println!("value is {}", value);
}

fn explicit_lifetime<'a>(left: &'a i32, right: &'a i32) -> &'a i32 {
    if left > right {
        left
    } else {
        right
    }
}

// compiler is happy with this because
// its smart to understand that the sending reference
// is the same scope when returned.
fn ok() {
    let referenced_int = 6;
    let returned_value = return_one_param(&referenced_int);
    println!("{}", returned_value);
}

fn return_one_param(value: &i32) -> &i32 {
    value
}

// fn bad2() {
//     // compiler won't let you do this as well.
//     let returned_ref = return_bad_ref();
// }

// fn return_bad_ref() -> &i32 {
//     let value = 5;
//     &value
// }

// fn bad() {

//     let outer_scope;
//     {
//         let inner_scope = 5;
//         outer_scope = &inner_scope;
//          // since inner scoe gets freed as soon as the
//          // the scope is over outer_scope becomes a dangling reference
//          // dangerous in C, but rust protects you.
//     }
//      // compiler won't let you use a dangling reference.
//     println!("outer scope: \t{}", outer_scope);
// }
