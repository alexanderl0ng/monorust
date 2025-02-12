fn main() {
    print_fahrenheit_to_celcius();
    print_fibonacci();
    print_12_days_of_christmas();
}

fn print_fahrenheit_to_celcius() {
    let fahrenheit = 350.0;
    let celcius = fahrenheit_to_celcius(fahrenheit);

    println!("Using the fahrenheit to celcius formula, {fahrenheit} in Celcius is: {celcius}");
}

fn fahrenheit_to_celcius(temp: f32) -> f32 {
    let celcius = (temp - 32.0) * (5.0 / 9.0);
    return celcius;
}

fn print_fibonacci() {
    let n = 15;
    let n_fibonacci = fibonacci(n);

    println!("The nth term of the Fibonacci sequence is: {n_fibonacci}");

}

fn fibonacci(n: u32) -> u32 {
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn print_12_days_of_christmas() {
    let numbers = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let number_lyrics = [
        "And a partridge in a peark tree.",
        "Two turtle doves,",
        "Three french hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
        ];

    for i in 0..numbers.len() {
        println!("On the {} day of Christmas,", numbers[i]);
        println!("my true love gave to me");
        if i == 0 {
            println!("A partridge in a pear tree.\n\n");
            continue;
        }
        for j in (0..i+1).rev() {
            println!("{}", number_lyrics[j]);
        }
        println!("\n");
    }
}
