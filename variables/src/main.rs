fn main() {

    // how to allow a variable unused to compile 
    // to not emit warnings.
    let _warning_variable = 0;

    // by default rust will not auto-cast and block division of 
    // incompatible types unlike other languages
    // you must specify the cast and accept the potential precision 
    // loss
    let float_thirty_two: f32 = 17.2;
    let unsigned_eight: u8 = 5;

    let _result = float_thirty_two / unsigned_eight as f32;
}
 