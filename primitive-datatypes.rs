//
// int, float, bool, char
// in: Signed integer can hold positive and negative values
// un: Signed integer can hold only positive values
fn main()
{
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed integer: {}", x);
    println!("Unsiigned integer: {}", y);

    let e: i32 = 2147483647; // -2^31 i32 - 2147483647
    let i: i64 = 9223372036854775807; // +2^63 i64 - 9223372036854775807
    println!("Max value of i32: {}", e);
    println!("Max of i64: {}", i);
    let pi : f64 = 3.14;
    println!("PI: {}", pi);
    let is_snowing: bool = true;
    println!("Is snowing: {}", is_snowing);
    let letter: char = 'A';
    println!("First letter of the alphabet: {}", letter)
    
    
}
