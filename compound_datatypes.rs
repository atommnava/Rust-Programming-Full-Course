// arrays, tuples, slices, and strings (slice string)

/*
    2 TYPES OF FORMATS
    - Debuggable
    - Display
 */
fn main()
{
              //[in; size]
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);
    let fruits: [&str; 3] = ["Apple", "Banana", "Coco"];
    println!("Fruits: {:?}", fruits);
    println!("Fruits [1]: {}", fruits[0]);
    println!("Fruits [2]: {}", fruits[1]);
    println!("Fruits [3]: {}", fruits[2]);

    // --------- Tuples -----------
            // (NOT NECESSARY TO SPECIFY)
    let human: (&str, i32, bool) = ("Alice", 30, true); 
    // At first, "Alice" is not a String
    // To convert it we use "Alice".to_string()
    // Homogeneuos = [1,2,3,4,5]
    // Non Homogeneous = ["Hi", 1, true, 0.2
    println!("Human Tuple: {:?}", human);
    let my_mix_tuple = ("Abel", 23, true, [1,2,3,4,5]);
    println!("Mix tuple: {:?}", my_mix_tuple);

    // --------- Slices -----------
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Num slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Wolf", "Gorilla"];
    println!("Animal slice: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"The Book Thief".to_string(),
                                    &"Les Misérables".to_string(),
                                    &"Anne Frank".to_string()];
    println!("Book Slice: {:?}", book_slices);

    // --------- String VS String Slices (&str) -----------
    // They are expandable, you can increase or decrease them if you wnat
    // MUTABLE

    //Any data type in Rust by defult is inmutable
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    // B- &str (String slice) Inmutable
    // Stack is quicker  | Can't have any mutable data types
    // Heap is slower | Dynamic mutable data types
    let string: String = String::from("Hello World!");
    let slice: &str = &string[..5];
    println!("Slice value: {}", slice) 
}