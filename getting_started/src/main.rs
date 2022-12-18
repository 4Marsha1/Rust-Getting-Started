fn main() {
    println!("Is it Rust!");
    
    // VARIABLES & DATA TYPES
    let name: &str = "John Wick";
    let age: i32 = 42;
    let mut is_true: bool = false;
    let salary: f32 = 1089023.2345;
    println!("I am {} and I am {}. I earn {}. {}!", name, age, salary, is_true);
    is_true = true;
    println!("I am {} and I am {}. I earn {}. {}!", name, age, salary, is_true);

    // IF_ELSE
    let n:i64 = 98;
    if n>100 {
        println!("{} is greater than 100", n);
    }else if n == 100 {
        println!("{} is equal to 100", n);
    }else {
        println!("{} is lesser than 100", n);
    }

    // LOOP
    let mut i = 0;
    loop{
        i+=1;
        if i==5{
            continue;
        }
        if i>10{
            break;
        }
        println!("Value of i is {}",i);
    }

    let mut j=0;
    while j<10{
        j+=1;
        if j%2==0{
        println!("The value of j is {}",j);
        }
    }

    for i in 1..9{
        println!("The value of i is {}",i);
    }

    // RANGE
    let range = 30..100;
    for i in range{
        println!("The value of i in RANGE is {}",i);
    }

    // VECTORS
    let names = vec!["John", "Wick", "Mike"];
    for (index, name) in names.iter().enumerate(){
        println!("The name in index {} is {}",index, name);
    }

    // ENUMS
    enum DIRECTION {
        UP,
        DOWN,
        LEFT,
        RIGHT,
    };
    let player_direction: DIRECTION = DIRECTION::LEFT;
    match player_direction{
        DIRECTION::UP => println!("Going UP"),
        DIRECTION::DOWN => println!("Going DOWN"),
        DIRECTION::LEFT => println!("Going LEFT"),
        DIRECTION::RIGHT => println!("Going RIGHT"),
    };

    // CONSTANTS
    const MAX_NUM: u8 = 30;
    for i in 1..30{
        if i%3==0{
            println!("{}", i);
        }
    }

    // TUPLES
    let tuple = (10, "RUST", (1, "Cool", true));
    let tuple2 = (10, "Cool", false);
    let (a,b,c) = tuple2;
    let (d,e,(f,g,h)) = tuple;
    let elem = tuple.1;
    println!("{} {} {} {} {}", d,e,f,g,h);
    println!("{} {} {} {}", a,b,c, elem);

    // FUNCTIONS
    print_numbers_upto_N(10);

    // SHADOWING
    let mut x=10;
    {
        let x = 15;
    }
    println!("{} is not mutated", x);

    // REFERENCES
    let mut y=100;
    let yr=&y;      // Immutable Ref
    let mutYr = &mut y; // Mutable Ref
    *mutYr+=9;
    // println!("{} is {}", y, yr);
    println!("y is updated {}", mutYr);

    // STRUCTS
    struct RGB{
        red: u8,
        green: u8,
        blue: u8
    };
    let mut bg = RGB {red:255, green:78, blue:189};
    println!("Red: {} Green:{} Blue:{}", bg.red, bg.green, bg.blue);
}

fn print_numbers_upto_N(num: i32){
    for i in 1..num+1{
        if is_even(i) {
            println!("{} is even", i);
        }else {
             println!("{} is odd", i);
        }
    }
}

fn is_even(x: i32) -> bool{
    return x%2==0;
}