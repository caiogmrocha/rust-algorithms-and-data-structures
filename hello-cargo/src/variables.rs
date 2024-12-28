fn variables() {
    // Variables, Values and Types
    // let integer_8_bits: i8 = 1;
    // let integer_16_bits: i16 = 1;
    let integer_32_bits: i32 = 1;
    // let integer_64_bits: i64 = 1;
    // let unsigned_integer_8_bits: u8 = 1;
    // let unsigned_integer_16_bits: u16 = 1;
    // let unsigned_integer_32_bits: u32 = 1;
    // let unsigned_integer_64_bits: u64 = 1;
    let float_32_bits: f32 = 1.5;
    // let float_64_bits: f64 = 1.5;
    let charr: char = 'A';
    // let boolean: bool = true;
    let tuple: (i32, char, f64) = (integer_32_bits.into(), charr, float_32_bits.into());
    // let array = [5,5];
    println!("Hello Tuple: {}", tuple.2);
}