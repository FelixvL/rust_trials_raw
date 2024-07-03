fn main() {
    main20();
 
}


use std::collections::HashMap;
struct Student {
    id: i32,
    name: String,
    grade: String,
}

struct StudentManager {
    students: HashMap<i32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        StudentManager {
            students: HashMap::new(),
        }
    }

    fn add_student(&mut self, student: Student) -> Result<(), String> {
        if self.students.contains_key(&student.id) {
            Err(format!("Student with ID {} already exists", student.id))
        } else {
            self.students.insert(student.id, student);
            Ok(())
        }
    }

    fn get_student(&self, id: i32) -> Option<&Student> {
        self.students.get(&id)
    }
}

fn main20() {
    let mut manager = StudentManager::new();

    let student1 = Student {
        id: 1,
        name: String::from("Alice"),
        grade: String::from("A"),
    };
    let student2 = Student {
        id: 2,
        name: String::from("Bob"),
        grade: String::from("B"),
    };

    manager.add_student(student1).unwrap();
    manager.add_student(student2).unwrap();

    // Retrieve and print student information
    if let Some(student) = manager.get_student(1) {
        println!("Student ID: {}", student.id);
        println!("Student Name: {}", student.name);
        println!("Student Grade: {}", student.grade);
    }
    if let Some(student) = manager.get_student(2) {
        println!("Student ID: {}", student.id);
        println!("Student Name: {}", student.name);
        println!("Student Grade: {}", student.grade);
    }
}


fn calculate_square(num: i32) -> Result<i32, String> {
    if num >= 0 {
        let result = num * num;
        println!("The square of {} is: {}", num, result);
        Ok(result)
    } else {
        Err("Negative number provided".to_string())
    }
}

fn main19() {
    let number = 7;
    if let Err(e) = calculate_square(number) {
        println!("Error: {e}");
    }
}



enum Measurement {
    CircleArea(f64),
    RectangleArea(f64, f64),
    TriangleArea(f64, f64),
    Perimeter(Vec<f64>),
}

impl Measurement {
    fn calculate(self) -> Result<f64, String> {
        match self {
            Self::CircleArea(radius) => {
                if radius < 0.0 {
                    Err(String::from("Radius cannot be negative"))
                } else {
                    Ok(std::f64::consts::PI * radius * radius)
                }
            }
            Self::RectangleArea(length, width) => {
                if length < 0.0 || width < 0.0 {
                    Err(String::from("Length and width cannot be negative"))
                } else {
                    Ok(length * width)
                }
            }
            Self::TriangleArea(base, height) => {
                if base < 0.0 || height < 0.0 {
                    Err(String::from("Base and height cannot be negative"))
                } else {
                    Ok(0.5 * base * height)
                }
            }
            Self::Perimeter(sides) => {
                if sides.len() < 3 {
                    Err(String::from("A polygon must have at least 3 sides"))
                } else {
                    Ok(sides.iter().sum())
                }
            }
        }
    }
}

fn main18() {
    let user_input = Measurement::TriangleArea(5.0, 8.0);
    match user_input.calculate() {
        Ok(res)=> println!("Result: {res}"),
        Err(e)=> println!("Error: {e}"),
    }
}


fn check_fruit(input_fruit: String) -> Option<String> {
    let fruit_basket = vec![
        String::from("mango"),
        String::from("apple"),
        String::from("banana"),
    ];
    for fruit in fruit_basket {
        if input_fruit == fruit {
            return Some(fruit);
        }
    }
    None
}

fn main17() {
    let user_fruit = String::from("apple");
    if let Some(fruit) = check_fruit(user_fruit) {
        println!("User's name: {fruit}");
    }
}


fn first_character(chars: &Vec<char>) -> Option<char> {
    if chars.len() > 0 {
        Some(chars[0])
    } else {
        None
    }
}

fn main16() {
    let my_chars = vec!['a', 'b', 'c', 'd'];
    match first_character(&my_chars) {
        Some(character) => println!("First character: {character}"),
        None => println!("Empty array"),
    }
}

enum Value {
    Integer(i32),
    Float(f32),
}

fn main15() {
    let some_val = vec![Value::Integer(12), Value::Float(15.5)];

    for i in some_val {
        match i {
            Value::Integer(num) => println!("Integer: {} ", num),
            Value::Float(num) => println!("Float: {}", num),
        }
    }
}


fn main14(){
    let mut mijnHuis = Huis{
        straatnaam: String::from("Vlakbij"),
        huisnummer: 25,
        prijs: 120000,
        tekoop: true
    };
    mijnHuis.verkopen();
    mijnHuis.prijsVerhogen(20000);
    mijnHuis.verkopen();
    let huis2 = mijnHuis.huisOpleveren();
    huis2.verkopen();
    println!("{}",Huis::WOZ());
    println!("{}",huis2.belastingBerekenen());
    let huis3 = Huis::new(String::from("daadoei"), 55);
    huis3.verkopen();
}

struct Huis{
    straatnaam: String,
    huisnummer: i32,
    prijs: i32,
    tekoop: bool
}

impl Huis{
    fn WOZ()->i32{
        125
    }
    fn belastingBerekenen(&self)->i32{
        self.prijs + Huis::WOZ()
    }
    fn new(snaam: String, huisnr:i32)->Self{
        Self{
            straatnaam:snaam,
            huisnummer: huisnr,
            prijs: 0,
            tekoop: false,
        }
    }
    fn verkopen(&self){
        println!(
            "Huis op {} {} is te koop voor {}",
            self.straatnaam, self.huisnummer, self.prijs 
        );
    }
    fn prijsVerhogen(&mut self, verhoging: i32){
        self.prijs += verhoging;
    }
    fn huisOpleveren(self)-> Self{
        self
    }
}



fn main13() {
    let mut first_num = 42;
    let mut second_num = 64;
    let ref1 = &mut first_num;
    let mut ref2 = &mut second_num; // a mutable references means that the reference can be updated to point to some other variable

    *ref1 = 15;
    *ref2 = 10;
    ref2 = ref1;

    println!("Updated first number: {ref2}");  
}

fn main12() {
    let mut vec_1 = vec![1, 2, 3];
    let vec_2 = vec![4, 5, 6];
    let mut vec_ptr: &Vec<i32>;
    vec_ptr = &vec_1; 
    println!("vec ptr is pointing to vec_1");
    vec_ptr = &vec_2; 
    println!("vec ptr is updated and now pointing to vec_2");
}



fn main11() {
    let mut some_vec = vec![1, 2, 3];
    let first = get_first_element(&some_vec);
    some_vec.push(4);
    println!("The first number is: {}", first);
}

fn get_first_element(num_vec: &Vec<i32>) -> i32 {
    num_vec[0]
}




fn main10() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let mut temp;

    while !my_vec.is_empty() {
        temp = &my_vec; // Something wrong on this line
        println!("Elements in temporary vector are: {:?}", temp);


        if let Some(last_element) = my_vec.pop() { // pop() is used to remove an element from the vec
            println!("Popped element: {}", last_element);
        }
    }
}


fn main9() {
    let s1: String = String::from("this is me, ");
    let s2: &str = "Nouman";
    some_function(s1.clone(), s2.clone()); // Something is wrong here
    println!("{} {}", s1, s2);
}

fn some_function(a1: String, a2: &str) {
    println!("{} {}", a1, a2);
}











fn mijnfn( s: &str){
    println!("{s}");
}
fn verm(num1:i32, num2:i32)->i32{
    println!("vermenigvuldigen{num1} * {num2}");
    num1 * num2
}
fn main2() {
    let x = 3;
    let y = 4;
    println!(
        "The result of x+3 times y+5 is {}",
        times(add_3(x), add_5(y))
    );
}
fn times(a:i32,b:i32)->i32{
    a * b
}
fn add_3(x:i32)->i32{
    x + 3
}
fn add_5(x:i32)->i32{
    x + 5
}
fn double(x: i32) -> i32 {
    x * 2
}

fn triple(x: i32) -> i32 {
    x * 3
}

fn main3() {
    println!("Answer: {}", triple(triple(double(5))));
}

fn print_distance(point: (f32, f32)) -> f32 {
    let (x, y) = point;
    (x.powf(2.0) + y.powf(2.0)).sqrt() // Formula for computing distance
}

fn main4() {
    println!(
        "The distance of the number the point from the origin is {}",
        print_distance((5.0, 4.0)) // concentrate on the call to the function 
    );
}
fn doublex(x: i32) -> i32 {
    x * 2
}

fn quadruple(x: i32) -> i32 {
    // your code here // 
    doublex(x) * 2 
}

fn main5() {
    println!(
        "For 1: the expected value is 4 while the output is {}",
        quadruple(1)
    );

    println!(
        "For 2: the expected value is 8 while the output is {}",
        quadruple(2)
    );

    println!(
        "For 3: the expected value is 12 while the output is {}",
        quadruple(3)
    );

    println!(
        "For 4: the expected value is 16 while the output is {}",
        quadruple(4)
    );
}

fn square_of_suma(getal : i32)->i32{
    let mut counter = 1;
    let mut temptotal = 0;
    let totaal = loop{
        if counter == getal+1{
            break temptotal * temptotal;
        }
        temptotal = temptotal + counter;
        counter = counter+1;
    };
    println!("Totaal in sq_o_sum{totaal}");
    totaal
}
fn sum_of_squaresa(getal : i32)->i32{
    let mut counter = 1;
    let mut temptotal = 0;
    let totaal = loop{
        if counter == getal+1{
            break temptotal;
        }
        temptotal = temptotal + (counter*counter);
        counter = counter+1;
    };
    println!("Totaal in sum_o_squ{totaal}");
    totaal
}

fn main6() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");

    let mut square_of_sum = 0;
    let mut sum_of_squares = 0;
    println!("opdracht 6 klaar");
    square_of_sum = square_of_suma(n);
    sum_of_squares = sum_of_squaresa(n);
    /* Complete the code after this line */
    let difff = square_of_sum - sum_of_squares;
    println!("{difff}")
}

fn main7() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input"); 
    let mut total = 0;
    /* Add your code below this line */   
    for x in 1..=n {
        if x % 3 == 0 || x % 5 == 0{
            println!("{x}");
            total = total + x;
        }
    }
    println!("Einde 7 = {total}")
    
}
fn main8(){
    let vec1: Vec<i32> = vec![1,2,3];
    takeownership(vec1.clone());
    println!("vec 1 is: {:?}", vec1);
    let vec2:Vec<i32> = givesownership();
    println!("vec 2 is: {:?}", vec2);
    let vec3:Vec<i32> = takes_and_gives_ownership(vec2);
    //println!("vec 2 is: {:?}", vec2);
    println!("vec 3 is: {:?}", vec3);

    
}
fn takeownership(vec:Vec<i32>){
    println!("vec in fn is: {:?}", vec)
}
fn givesownership()->Vec<i32>{
    vec![4,5,6]
}
fn takes_and_gives_ownership(mut vec:Vec<i32>)->Vec<i32>{
    vec.push(10);
    vec
}
