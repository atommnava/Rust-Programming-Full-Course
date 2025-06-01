fn main()
{
    let age: i32 = 20;
    if age >= 18 {
        println!("You can drive!");
    } else if age < 18{
        println!("You cannot drive...");
    } else {
        println!("How old are u?");
    }

    let condition = true;
    let number = if condition {5} else {6};
    println!("Number: {number}");
}