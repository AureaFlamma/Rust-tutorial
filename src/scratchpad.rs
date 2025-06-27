fn main() {
    println!("-> what is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Failed to read line");
    println!("-> Hello {}! {}", name.trim_end(), greeting);
}

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}

// generic loop. What's the JS equivalent?
fn main() {
    let arr_2 = [1,2,3,4, 5, 6];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx +=1;
            continue;
        }
        if arr_2[loop_idx] == 9{
            break;
        }
        println!("val : {}", arr_2[loop_idx]);
        loop_idx +=1;
    }
}

// while loop
/*
- **Rust's `while`** === **JS's `while`** - conditional loops
- **Rust's `loop`** === **JS's `while(true)`** - infinite loops
*/
fn main() {
    let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;
    while loop_idx < arr_2.len(){
        println!("Arr : {}", arr_2[loop_idx]);
        loop_idx +=1;
    }
}

fn main() {
    let arr_2 = [1,2,3,4,5,6,7,8,9];
    for val in arr_2.iter() {
        if *val == 4 {
            println!("lmao")
        } else {
            println!("Val : {}", val)

        }
    }
}

fn main() {
    let arr_2 = [1,2,3,4,5,6,7,8,9];
    for val in arr_2.iter() {
        if *val == 4 { // the star is necessary because of the iter. Otherwise, we wouldn't need it
            println!("lmao")
        } else {
            println!("Val : {}", val)

        }
    }
}

fn main() {
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    let(v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);
}

fn main() {
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    let(v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);
}

fn main() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace(){
        println!("{}", word);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2)
}

fn main() {
    let st3 = String::from("x f w a g b c y");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char)
    }
}

fn main() {
    let st4: &str = "Random string";
    let mut st5 = st4.to_string();
    println!("{}", st5);
}

fn main() {
    let st4: &str = "Chihuauah string";
    let mut st5 = st4.to_string();
    
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length : {}", st6.len());
    println!("dafuq is this? {}", st6); // prints: "Chihua"
}

fn main() {
    let st4: &str = "Random string";
    let mut st5 = st4.to_string();
    println!("here's the uncleared string : {}", st5);

    st5.clear();
    println!("here's the cleared string : {}", st5);
}

fn main() {
    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;
    for char in st8.chars(){
        println!("{}", char)
    }
}

fn main() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
}

fn main() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day:: Sunday => true,
                _ => false
            }
        }
    }

    let today:Day = Day::Monday;
    match today {
        Day::Monday => println!("Everyone hates Mondays"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("almost Weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }

    println!("Is today the weekend {}", today.is_weekend());

}


fn main() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];
    vec2.push(5); // it can change its length, without changing its type
    println!("1st : {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1){
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd value"),
    }
    for i in &mut vec2 {
        println!("Reference address: {:p}", i);   // Shows memory address
        println!("Actual value: {}", *i);         // Shows the value
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }
    println!("Vec Length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());
}

fn get_sum(x: i32, y: i32) -> i32 {
    x + y
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    return x + y;
}

fn get_2(x: i32) -> (i32, i32) {
    return (x+1, x+2)
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter(){
        sum += &val;
    }
    sum
}

fn main() {
    println!("{}", get_sum(1, 2));
    println!("{}", get_sum_2(3, 4));
    let (val_1, val_2) = get_2(3);
    println!("Nums : {}, {}", val_1, val_2);
    let num_list = vec![1,2,3,4,5];
    println!("Sum of list = {}", sum_list(&num_list));
}



use std::ops::Add;

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn main() {
    println!("5 + 4 = {}", get_sum_gen(5, 4)); // works with integers
    println!("5.2 + 4.6 = {}", get_sum_gen(5.2, 4.6)); //works with floats
}

// OWNERSHIP

fn main() {
    let str1 = String::from("World");
    let str2 = str1.clone();
    let str3 = str1;
    println!("Hello {}", str1); // errors - tries to borrow a moved value
    println!("Hello {}", str2); // works fine
    println!("Hello {}", str3); // works fine
    
}

fn main() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    // you can iterate over hash maps
    for(k, v) in heroes.iter(){
        println!("{} = {}", k, v);
    }

    // Hashmaps have length
    println!("Length : {}", heroes.len());

    if heroes.contains_key(&"Batman"){
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }
}

fn main() {
    // Struct is basically a mixed-type hashmap
    struct Customer{
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer{
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"),
        balance: 234.50
    };

    bob.address = String::from("505 Main St");
}

fn main() {
    // generic structs allow us some flexibility as to the types
    struct Rectangle<T, U> {
        length: T,
        height: U,
    }

    let rec = Rectangle {
        length: 4,
        height: 10.5
    };

    let rec2 = Rectangle {
        length: "A lot",
        height: "also a lot"
    };

    struct Generic<T> {
        prop1: T,
        prop2: T,
    };

    let generic1 = Generic {
        prop1: "a",
        prop2: "b"
    };

    let generic2 = Generic {
        prop1: 6,
        prop2: "a" //this errors
    };


}

fn main() {
    const PI: f32 = 3.141592;
    // Shape is a _trait_ - it defines the general data types ( the contract)
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {length: f32, width: f32};
    struct Circle {length: f32, width: f32};
    // Rectangle and Circle are structs (concrete types) - they define the specific mechanics of e.g. how area is calculated.
    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }
    // rec and circ are variables; their values are instances of the struct
    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(5.0, 5.0);

    println!("Area of Rectangle, {}", rec.area());
    println!("Area of Circle: {}", circ.area());
}

// WRITING TO FILE AND ERROR HANDLING

fn main() {
    let path = "lines.txt";

    // Result has 2 varients Ok and Err
    // enum Result<T, E> {
    // Ok(T),
    // Err(E), }
    // Where T represents the data typeof the value returns and E
    // the type of error

    // CREATE FILE
    let output = File::create(path);
    // One way of handling errors
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file : {:?}", error);
        }
    };

    // WRITE TO FILE
    // Another way of handling errors. Panic with custom message when error occurs.
    write!(output, "Just some \nRandom words").expect("Failed to write to file");

    // READ FILE
    // Third way of handling errors. Unwrap panics with default message if error occurs. 
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

/* 
// These are equivalent:
let input = File::open(path).unwrap();

// Same as:
let input = match File::open(path) {
    Ok(file) => file,
    Err(error) => panic!("called `Result::unwrap()` on an `Err` value: {:?}", error),
};
*/

}

fn main() {
    // ErrorKind example:
    // Tries to create a file; if ok, stores the file in a variable
    // If Error, checks error kind:
    // if Error kind is NotFound (unlikely for create, but possible), tries to create file under different name
    // If successful, stores that file in a var
    // if not successful, panics with custom error "Can't create file".
    // If the parent error is of any other kind than NotFound, panics with custom error "Problem opening file"
    
    let output = File::create("rand.txt");
    let output = match output {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create
            ("random.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", e),
            },
            _other_error => panic!("Problem opening file : {:?}", error),
        },
    };

}

fn main() {
    // Iteration borrows values
    let mut arr_it = [1,2,3,4];
    for val in arr_it.iter(){
        println!("{}", val);
    }
    let mut iter1 = arr_it.iter();
    println!("1st : {:?}", iter1.next());
}

// CLOSURE
fn main() {
    let mut samp1 = 5;
    let print_var_closure = || println!("samp1 = {}", samp1);
    fn print_var_function() {
        println!("samp1 = {}", samp1);
    }
    print_var_closure(); // This prints
    print_var_function(); // This errors
}