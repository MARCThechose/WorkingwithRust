fn calculate_sum(limit: u32) -> u32 
{
    let mut sum = 0;   
    
    for i in 1..=limit 
    {
        sum += i; 
    }
    sum
}

fn main() 
{
    let input_value = 10;
    let result = calculate_sum(input_value);
    println!("The sum of numbers frtom 1 to {} is: {}", input_value, result);
    panic!("Intentional panic for demonstration purposes.");
}