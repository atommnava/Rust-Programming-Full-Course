fn main()
{
    /*
     * Ownership Rules
     * 1. Each value in Rust has an owner
     * 2. There can be only one owner at a time
     * 3. When the owner goes out of scope, the value will be dropped
     */
     // 1. Rule
     //let s1 = String::from("RUST"); // s1 is the owner of that string
     /*
     let len = calculate_length(&s1);
     println!("Lenght of '{}' string is: {}", s1, len);
     */

     // 2. Rule
     /*
     let s2 = s1;
     println!("S2: {}", s2);
     */
     // 3. Rule
     let s1 = String::from("Atom");
     let len = calculate_length(&s1);
     println!("Lenght of '{}' string is: {}", s1, len);
     // s1 goes out of scope and its value will be dropped
     
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn print_lost(s: &string)
{
    println!("{}", &s1);
}

