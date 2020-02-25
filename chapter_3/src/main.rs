fn main() {
    days_of_christmas();

    let temp: f32 = 50.0;
    
    println!("Temp in fahrenheit: {}", temp);
    println!("Temp in celsius: {}", fahrenheit_to_celsius(temp));

    println!("Temp in celsius: {}", temp);
    println!("Temp in fahrenheit: {}", celsius_to_fahrenheit(temp));
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    c * 1.8 + 32.0
}

fn days_of_christmas() {
    let gifts = [
        "partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "golden rings",
        "geese a laying",
        "swans a singing",
        "maids a milking",
        "ladies dancing",
        "lords a leaping",
        "pipers piping",
        "drummers drumming"
    ];

    for day in 1..13 {
        let cont = match day {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };

        println!("On the {}{} day of Christmas my true love gave to me:",day,cont);

        for rev in (1..day + 1).rev() {
            if rev == 1 {
                if rev == day {
                    println!("a {}", gifts[rev-1]);
                }
                else {
                    println!("and a {}", gifts[rev-1]);
                }
            }
            else {
                println!("{} {}",rev, gifts[rev-1]);
            }
        }

        println!();
    }

    
}
