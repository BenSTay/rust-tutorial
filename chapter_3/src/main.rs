fn main() {
    days_of_christmas();
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
