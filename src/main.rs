/*
Going a bit deep about the data types.
Checking the integer.
Integer have 5 types i8, i16, i32, i164, i128
Now just have a look about the max and min number possible for any type.
*/

/*
Will need it later.
Look how to declare a function.
First of all key-word then the function name and then the arguments of the functions with them type.
Then we have to tell the return type of our functions, and we do that with ->i32 in this case.
At the end we can build our instruction.
*/

use std::fmt;

fn reverse(pair:(bool, i32))->(i32, bool){
    let (boolean,integer) = pair;
    (integer, boolean)
}

fn rectangle_area(n1:f32, n2:f32)->(f32){
    (n1 * n2) / 2.0
}

const LIMITS:i32 = 2004;


fn is_bigger(n:i32)-> bool{
    n >= 18
}


fn sum(n:i32, n1:i32) -> i32{ n+ n1 }

fn less(n2:i32, n3:i32) -> i32{
    n2 - n3
}

fn div(n4:i32, n5:i32) -> i32{ n4 / n5 }

fn foor(n6:i32, n7:i32) -> i32{
    n6 / n7
}

struct People{

    name: String,
    surname: String,
    age: i8
}


fn cnc(s1: String, s2: String)-> String{
    s1 + &s2
}

fn main() {
    // SOMETHING OUT OF THIS
    /*
    We can even pass numbers in this way, just to work with numbers more familiar with us
    */

    let number = 10_000_000; //more easy to read than 10000000
    println!("{}", number);
    println!("");
    let normal_number = 10000000;
    println!("{}", normal_number);
    println!("");

    //Starting Here

    let n1: i8 = 10;
    let n2: i16 = 10;
    let n3: i32 = 10;
    let n4: i64 = 10;
    let n5: i128 = 10;

    //Output will be 127, -128
    let (max1, min1) = (i8::MAX, i8::MIN);
    println!("i8 Max is {}, Min is {}", max1, min1);

    //Output will be 32767, -32768
    let (max2, min2) = (i16::MAX, i16::MIN);
    println!("i16 Max is {}, Min is {}", max2, min2);

    //Output will be 2147483647, -2147483648
    let (max3, min3) = (i32::MAX, i32::MIN);
    println!("i32 Max is {}, Min is {}", max3, min3);

    //Output will be 9223372036854775807, -9223372036854775808
    let (max4, min4) = (i64::MAX, i64::MIN);
    println!("i64 Max is {}, Min is {}", max4, min4);

    //Output will be 170141183460469231731687303715884105727, - 170141183460469231731687303715884105728
    let (max5, min5) = (i128::MAX, i128::MIN);
    println!("i128 Max is {}, Min is {}", max5, min5);
    println!("");

    /*
    Now juast have a look about float.
    Float have only 2 types: f32, f64
    Just have a look about it
     */

    let f1: f32 = 10.24;
    let f2: f64 = 10.24;

    //Output will be 340282350000000000000000000000000000000, -340282350000000000000000000000000000000
    let (maxf1, minf1) = (f32::MAX, f32::MIN);
    println!("f32 Max is {}, Min is {}", maxf1, minf1);
    println!("");

    //Output will be (Result is too big so check on your prompt)
    let (maxf2, minf2) = (f64::MAX, f64::MIN);
    println!("f64 Max is {}, Min is {}", maxf2, minf2);
    println!("");

    // Passing in Char.
    /*
    Not too much to say, you can call it in this way and you can pass it whatever you want, even an emoji.
    Just look about the ' '.
    To declare a char we will need of single quotes
     */

    let c1: char = ('💡');
    println!("{}", c1);
    println!("");

    /*
    Now just have a look about functions().
    functions() are pieces of code that we can call as many times as we want.
    We use it when we have some instruction to repeat.
    Now we want a simple sum function, but it's just to understand how it works.
    function is on top.
    After the explaination of the functions, as we said before we can call the function every time we want,
    just passing same 2 arguments.
    */

    let my_number: i32 = 10;
    let my_second_number: i32 = 15;
    println!("Sum is {}", sum(my_number, my_second_number));

    //or just
    let summmm = sum(10, 54);
    println!("Sum is {}", summmm);
    println!("");


    // INPUT

    let logo: char = ('📲');
    println!("INPUT {}", logo);
    println!("");

    // Have a look about how input works.
    /*
    First of all we declare a mutable variable and we attach the integrated method String::new().
    This method will set the input.
    Remember, we can do only for strings if we want a number we have to convert it.
    With String::new() we are creating a new empty string to set, what the client will type.
    Then we want to print the question for the client.
    Then to receive and set the input we have to work in this way.
    First of all pass the module for input (io = input/output), then
    we use the read_line integrated module and we set our mutable variable, the we pass the unwrap
    integrate function
     */

    //We create this type of string String::new() when we are waiting for an input from the customer.
    let mut new_line = String::new();
    println!("Tell me your name: ");
    std::io::stdin().read_line(&mut new_line).unwrap();
    println!("Your name is {}", new_line);

    //Bit more complex to convert value from string to number

    let mut name = String::new();
    let mut age = String::new();
    let hundred = 100;

    println!("Tell me your name ---> ");
    std::io::stdin().read_line(&mut name).unwrap();
    println!("Tell me your age ---> ");
    std::io::stdin().read_line(&mut age).unwrap();
    let take_age: i32 = age.trim().parse().unwrap();
    println!("Your name is {}, and your age is {}", name, take_age);

    //Just adding some if-else condition.
    /*
    If we try to pass age instead take_age we will have an error.
    This happen because age is a string and take_age not anymore, in fact we convert it in integer with
    .trim().parse().unwrap()

    // This will not work
    if 100 - age <= 100 % 2{
        println!("You are old");
    }else{
        println!("You are young");
    }
     */

    if 100 - take_age <= 100 % 2 {
        println!("You are old");
    } else {
        println!("You are young");
    }


    // 2 BASIC EXERCISE.
    /*

    1 Create a program that will ask to the Customer input as: name, surname, job, age
    print everything with a good format.
     */

    let mut name = String::new();
    let mut surname = String::new();
    let mut job = String::new();
    let mut age = String::new();

    println!("Tell me your name ---> ");
    std::io::stdin().read_line(&mut name).unwrap();
    println!("");
    println!("Tell me your surname ---> ");
    std::io::stdin().read_line(&mut surname).unwrap();
    println!("");
    println!("Tell me your job ---> ");
    std::io::stdin().read_line(&mut job).unwrap();
    println!("");
    println!("Tell me your age --->");
    std::io::stdin().read_line(&mut age).unwrap();
    let set_age: i32 = age.trim().parse().unwrap();
    println!("Your Name is {}\nYour Surname is {}\nYour Job is {}\nYour age is {}", name, surname, job, age);

    /*
    2 Create a program that will ask 2 i32 input to the customer, then create 5 function for the
    basic math operation.
    */

    let mut first_number = String::new();
    let mut second_number = String::new();
    println!("Ok Chose your first number --> ");
    std::io::stdin().read_line(&mut first_number).unwrap();
    println!("");
    println!("Chose the second number --> ");
    std::io::stdin().read_line(&mut second_number).unwrap();
    let mut convert_first: i32 = first_number.trim().parse().unwrap();
    let mut convert_second: i32 = second_number.trim().parse().unwrap();
    if convert_second == 0{
        println!("Cannot divided by zero");
    }else if(convert_second > convert_first){
        println!("Cannot do some operations if second number is bigger than the first ");
    }else{
        let my_sum = sum(convert_first, convert_second);
        let my_less = less(convert_first, convert_second);
        let my_foor = foor(convert_first, convert_second);
        let my_div = div(convert_first, convert_second);
        println!("Result are:\nSum {}\nLess {}\nFor {}\nDiv {}", my_sum, my_less, my_foor, my_div);
        println!("");
    }


    // match
    let city = "London";
    let try_to_match = match city{
        "London" => {println!("Found it!"); "London"},
        "Berlin" => {println!("Found it!"); "Berlin"},
        "Rome" => {println!("Found it!"); "Rome"},
        _ => "Unknow",
    };
    println!("City is {}", try_to_match);
    println!("");

    // EXERCISE

    /*
    Create a calculator with all the basic operations and with statment if and else.
     */

    while true {
        println!("Hello everyone to my calculator");
        let mut first_choice = String::new();
        println!("Press 1 for Add, 2 for Sub, 3 for Mol, 4 for Div ----> ");
        std::io::stdin().read_line(&mut first_choice).unwrap();
        let convert_input_first_choice: i32 = first_choice.trim().parse().unwrap();

        if convert_input_first_choice == 1 {
            println!("You Chose Addition");

            let mut add_first_input = String::new();
            println!("Give me the first number --> ");
            std::io::stdin().read_line(&mut add_first_input).unwrap();
            let convert_add_first_input: f32 = add_first_input.trim().parse().unwrap();

            let mut add_second_input = String::new();
            println!("Give me the second number --> ");
            std::io::stdin().read_line(&mut add_second_input).unwrap();
            let convert_add_second_input: f32 = add_second_input.trim().parse().unwrap();

            let result = convert_add_first_input + convert_add_second_input;
            println!("Result is {}", result);
        } else if (convert_input_first_choice == 2) {
            println!("You choose Substraction");

            let mut sub_first_input = String::new();
            println!("Tell me the first number --> ");
            std::io::stdin().read_line(&mut sub_first_input).unwrap();
            let convert_sub_first_input: f32 = sub_first_input.trim().parse().unwrap();

            let mut sub_second_input = String::new();
            println!("Tell me the second number --> ");
            std::io::stdin().read_line(&mut sub_second_input).unwrap();
            let convert_sub_second_input: f32 = sub_second_input.trim().parse().unwrap();

            let result = convert_sub_first_input + convert_sub_second_input;
            println!("Result is {}", result);
        } else if (convert_input_first_choice == 3) {
            println!("Perfect You choose Moltiplication");

            let mut mol_first_input = String::new();
            println!("Tell me the first number --> ");
            std::io::stdin().read_line(&mut mol_first_input).unwrap();
            let convert_mol_first_input: f32 = mol_first_input.trim().parse().unwrap();

            let mut mol_second_input = String::new();
            println!("Tell me the second number --> ");
            std::io::stdin().read_line(&mut mol_second_input).unwrap();
            let convert_mol_second_input: f32 = mol_second_input.trim().parse().unwrap();

            let result: f32 = convert_mol_first_input * convert_mol_second_input;
            println!("Result is {:.2}", result);
        } else if (convert_input_first_choice == 4) {
            println!("You choose a Division");

            let mut first_div_input = String::new();
            println!("Tell me your first number --> ");
            std::io::stdin().read_line(&mut first_div_input).unwrap();
            let convert_div_first_input: f32 = first_div_input.trim().parse().unwrap();

            let mut second_div_input = String::new();
            println!("Tell me second number --> ");
            std::io::stdin().read_line(&mut second_div_input).unwrap();
            let convert_div_second_input: f32 = second_div_input.trim().parse().unwrap();

            if convert_div_second_input <= 0.0 {
                println!("Cannot make a division for 0 or less than 0");
                let mut second_question = String::new();
                println!("Do you want to change your choise? Y/N --> ");
                std::io::stdin().read_line(&mut second_question).unwrap();
                let convert_second_question = second_question.as_str();
                for i in convert_second_question.chars() {
                    if i == 'Y' || i == 'y' {
                        println!("Good idea");
                        let mut second_choise_in_second_question = String::new();
                        println!("Ok Tell me your number --> ");
                        std::io::stdin().read_line(&mut second_choise_in_second_question).unwrap();
                        let convert_second_choise_in_second_question: f32 = second_choise_in_second_question.trim().parse().unwrap();

                        let result = convert_div_first_input / convert_second_choise_in_second_question;
                        println!("Result is {}", result);
                    }
                };

                for i in convert_second_question.chars() {
                    if (i == 'N' || i == 'n') {
                        println!("Ok!");
                    }
                }
            } else {
                let result = convert_div_first_input / convert_div_second_input;
                println!("Result is {}", result);
            }
        }break
    }
    println!("");


    // For cicle
    for x in 1..50{
        println!("{}", x);
    }
    println!("");

    // Decalaring array and print it

    let mut my_array = [0, 1, 2, 3, 4, 5];

    // Can be declared even like this passing type and lenght of array in this way :[i32; 6] = [0, 1, 2, 3, 4, 5]
    let mut my_second_array:[i32; 6] = [0, 1, 2, 3, 4, 5];

    //Now we want to print and we can use a for iteration.
    //In the first iteraction if x is equal to 3 or 4 we don't print number, we skip it.

    for x in my_array{
        if x == 3 || x == 4{
            continue;
        }
        println!("Skipped 3 and 4 from array {}", x);
    }
    println!("");

    //Just try even for the second array

    for y in my_second_array{
        println!("Y second array is: {}", y);
    }
    println!("");

    // To print only a specific element we can use index like this.

    println!("This is the second element of the first array {}", my_array[1]);
    println!("");

    //To know the array lengt

    println!("Array Lenght is: {}",my_array.len());
    println!("");

    // To change an element

    my_array[1] = 27;

    for x in my_array {
        println!("Number changed {}", x);
    }

    println!("");

    // To print an array we have to do this.
    // Before we where printing element by element the array, now we are printing the array

    println!("My array is: {:?}", my_array);
    println!("");

    /*
    We can take a slice from array in this way, first we pass the & operator, then the name of the array,
    then [starting point..end point]
     */
    let my_slice = &my_array[1..4];
    println!("This is your slice of array {:?}", my_slice);
    println!("");

    // We can do the same for a string

    let my_string = "Hello Boys and Girls!";
    let my_second_slice = &my_string[3..12];
    println!("This is the string slice {:?}", my_second_slice);
    println!("");

    //Even this will be printed out, this is the last method to print a slice.
    println!("Last method to print a slice {:?}", &my_array[1..4]);
    println!("");


    // Tuple

    let my_tuple:(&str,i32) = ("Rust", 2022);
    println!("Second element of the tuple is {}", my_tuple.1);
    println!("");

    // Vector are similar to array but the difference is, with vector we can add o remove element.

    let mut vec1 = vec![0, 1, 2, 3, 4, 5];
    println!("{:?}", vec1);
    println!("Now we are adding 6");
    vec1.push(6);
    println!("Now vector is {:?}", vec1);
    println!("Now we are changing the second vector element");
    vec1[1] = 12;
    println!("After Change {:?}", vec1);
    println!("Now we are printing the second vector element");
    println!("Second element is {}",vec1[1]);
    println!("Now we will delete the last element");
    vec1.pop();
    println!("Now the vector is at the beginning {:?}", vec1);
    vec1.pop();
    println!("Now the vector is under value {:?}", vec1);
    println!("Now we want to delete everything");
    vec1.clear();
    println!("After .clear() vector is, {:?}",vec1);
    println!("");

    // Using a struct setted on top. just have a look about the name of the struct is
    // setted in a CamelCase.

    let person1 = People{
        name:String::from("Matteo"),
        surname:String::from("Gigli"),
        age:30
    };
    println!("This is the name of person1: {}", person1.name);
    println!("This is the surname of person1: {}", person1.surname);
    println!("This is the age of person1: {}", person1.age);
    println!("");

    //Working with strings

    let first_string = "Heeeeellooo Boys and Girls !";
    println!("String is {}", first_string);

    //You can ask for the string lenght
    println!("String Lenght is {}", first_string.len());

    //To change a part of string
    let second_string = first_string.replace("Boys", "Guys");
    println!("Second string is {}", second_string);

    //To delete and add a part of string
    let mut third_string = "My name is Matteo Gigli .";
    println!("Third string is {}",third_string);
    let mut change_string = third_string.replace(".", "");
    println!("Now After .replace() string looks like {}", change_string);
    change_string.push_str("And I'm 30");
    println!("Third string after .push_str() now is {}", change_string);

    // If we want to split for position we can do this
    let mut newest_string = "Hello Boys and Girls, My name is Matteo Gigli and I'm 30";
    let (newest_string_splitted ,newest_string_splitted2) = newest_string.split_at(19);
    println!("{0} {1}", newest_string_splitted, newest_string_splitted2);
    println!("");

    //Or we can itereate with for, for the space like this
    for i in newest_string.split(" "){
        println!("Split in ' ' {}", i);
    }
    println!("");

    for i in newest_string.split("B"){
        println!("Split in B: {}", i)
    }
    println!("");

    //Or we can split via \n
    println!("Split via n");
    println!("Hello\nGuys\nhow\nare\nyou?");
    println!("");


    //Or for single letter

    for j in newest_string.chars(){
        println!("{}", j);
    }

    println!("");

    //EXERCISE
    /*
    Ask a string as input, and than print chars one by one.
     */

    let mut first_ex_string = String::new();
    println!("Tell me your first string ->");
    std::io::stdin().read_line(&mut first_ex_string).unwrap();
    for i in first_ex_string.chars(){
        println!("{}", i);
    };

    //EXERCISE 2
    /*
    Ask 2 input to the customer then print out string lenght and concatenate string.
    */
    let mut first_string_second_ex = String::new();
    println!("Tell me the first here -> ");
    std::io::stdin().read_line(&mut first_string_second_ex).unwrap();
    let first_string_lenght = first_string_second_ex.len();
    println!("Lenght of first string is {}", first_string_second_ex.len());

    let mut second_string_second_ex = String::new();
    println!("Tell me the second here ->");
    std::io::stdin().read_line(&mut second_string_second_ex).unwrap();
    let second_string_lenght = second_string_second_ex.len();
    println!("Lenght of second string is {}", second_string_second_ex.len());

    println!("Lenght Result of 2 strings is {}",first_string_lenght + second_string_lenght);
    println!("");

    //Can concatenate like this
    let togheter = format!("{}{}", first_string_second_ex, second_string_second_ex);
    println!("{}",togheter);

    //Or like this with a function declared on top
    let togheter1 = cnc(first_string_second_ex, second_string_second_ex);
    println!("{}",togheter1);

    //Different Way to print
    println!("{subject}, {verb}, {object}",
             subject = "I",
             verb = "Am",
             object = "A Junior Developer");
    println!("");

    println!("{} of {:b} people know binary, the other doesn't", 1, 2);
    println!("");

    //Some space from the margin left...
    println!("{number:>width$}", number = 10, width = 6);
    println!("");


    //Response to final question in https://doc.rust-lang.org/stable/rust-by-example/hello/print.html

    //Assignement
    println!("My name is {0}, {1} {0}", "Bond", "James");
    println!("");

    //Struct
    // fmt::Debug trait Allow every type to be printable. derive attribute create automatically
    // the implementation to be printable from ftm::Debug

    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct `{:#?}` will be printed...", Structure(18)); // {:#?} nice style printed
    println!("");

    //Two decimals to show
    let pi: f64 = 3.141592;
    println!("First two decimals to show {:.2}", pi);
    println!("");

    /*
    fmt::Debug looks no too clean, so we can change our output style with the basic implementation of
    fmt::Display.
    set a structure than this is the basic implementation to use fmt::Display.
    write is like println, just passing the argumets with self.index of the arguments of your struct
     */
    struct MinMax(i32, i32, i64, i64);
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
        }
    }

    let minmax = MinMax(30, 50, 36, 48);
    println!("Display: {}", minmax);

    println!("");

    //Using a struct to associate 3 input from a client, then print them nice.

    #[derive(Debug)]
    struct State {
        city: String,
        where_do_you_live: String,
        distance_between_them: f32
    }
    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {}, {})", self.city, self.where_do_you_live, self.distance_between_them)
        }
    }

    let mut asking_info1 = String::new();
    println!("Which is your Capital? ---> ");
    std::io::stdin().read_line(&mut asking_info1).unwrap();
    let convert_asking_info1 = asking_info1.to_string();

    let mut asking_info2 = String::new();
    println!("Now tell me the city where you live ---> ");
    std::io::stdin().read_line(&mut asking_info2).unwrap();
    let convert_asking_info2 = asking_info2.to_string();

    let mut asking_info3 = String::new();
    println!("Ok now tell me the distance between them ---> ");
    std::io::stdin().read_line(&mut asking_info3).unwrap();
    let convert_third_output: f32 = asking_info3.trim().parse().unwrap();

    let all_in_one = State {
        city: convert_asking_info1,
        where_do_you_live: convert_asking_info2,
        distance_between_them: convert_third_output
    };

    print!("{:#?}", all_in_one);
    println!("");


    //Reverse a tuple with function reversed setted on top
    let tuple1 = (true, 3);
    println!("{:?}",reverse(tuple1));
    println!("");



    //Nice output and nuber setted on max of 2 decimals
    #[derive(Debug)]
    struct NewStructureProve {
        number1:f64,
        number2:f64,
        number3:f64
    }
    impl fmt::Display for NewStructureProve{
        fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
            write!(f,"Display: ({:.2} {:.2} {:.2})", self.number1, self.number2, self.number3)
        }
    }

    let get_all = NewStructureProve{number1:30.2365, number2:30.9645, number3:27.454567};
    println!("{:?}", get_all);
    println!("");

    // Writing 500 of the number passed
    let ys: [i32; 500] = [0; 500];
    println!("{:?}", ys);
    println!("");



    #[derive(Debug)]
    struct Rectangle{
        x: f32,
        y: f32,
        z: f32
    }

    impl fmt::Display for Rectangle{
        fn fmt(& self, f:&mut fmt::Formatter)->fmt::Result{
            write!(f,"({} {} {})", self.x, self.y, self.z)
        }
    }

    let mut rect_question1 = String::new();
    println!("Tell me the base --> ");
    std::io::stdin().read_line(&mut rect_question1).unwrap();
    let convert_rect_question1:f32 = rect_question1.trim().parse().unwrap();

    let mut rect_question2 = String::new();
    println!("Tell me the second number --> ");
    std::io::stdin().read_line(&mut rect_question2).unwrap();
    let convert_rect_question2:f32 = rect_question2.trim().parse().unwrap();

    println!("Your rectangle area is {}", rectangle_area(convert_rect_question1, convert_rect_question2));
    println!("");


    //Using outside const LIMITS and and function is_bigger document example

    let mut doc_question1 = String::new();
    println!("Hello do you have 18? Y/N ");
    std::io::stdin().read_line(&mut doc_question1).unwrap();
    let convert_response1 = doc_question1.as_str();

    for i in convert_response1.chars(){
        if i == 'Y' || i == 'y' {
            let mut confirmation = String::new();
            println!("Ok! Tell me your year of birth --> ");
            std::io::stdin().read_line(&mut confirmation).unwrap();
            let conversion_response2: i32 = confirmation.trim().parse().unwrap();
            let operation: i32 = LIMITS - conversion_response2;
            println!("After a check i can tell {} ", if is_bigger(operation) {"Go back home"} else {"You can go in"});

        }else if(i == 'n' || i == 'N'){
            println!("Ok You cannot go in");
        }else{
            "Don't Understand";
        };
    }

    //We can declare first a variable and then assign the value like this
    let a:i32;
    a = 4*4;
    println!("{}", a);




}

