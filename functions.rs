// snake case: hello_world
// kebab case: hello-world

/*
Writing the function below the main functions is not available in all programming languages.
You should define your function first and then call it afterwards.
In programming this is called HOISTING where you can actually define your function where you want.
 */
/*
fn hello()
{
    println!(":)");
}
*/

fn tell_height(height: i32)
{
    println!("My height is: {} cm", height);
}

fn human_id(name: &str, age: u32, height: f32)
{
    println!("My name is {}, I am {} years old, and my height is {} cm", name, age, height);
}

fn add(a: i32, b: i32) -> i32 // equivalent to int add(int a, int b){...}
{
    a + b // return a + b; equivalent
}

// BMI = height(kg)/height(m)^2
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64
{
    weight_kg / (height_m * height_m)
}

fn main()
{
    //hello();
    tell_height(182);
    human_id("Atom", 20, 172.0);

    let x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 5;
        price * qty
    };
    println!("Result is {}", x);
    println!("{}", add(2,7));
    let y: i32 = add(1,1);
    println!("y = {}", y);

    // CALLING BMI FUNCTION
    let weight: f64 = 70.0;
    let height: f64 = 1.72;
    let bmi: f64 = calculate_bmi(weight, height);
    println!("Data BMI: {:.2}", bmi); 
}

