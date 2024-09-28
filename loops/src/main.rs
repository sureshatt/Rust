fn main() {
    // loops
    let mut counter: i8 = 0;

    // loop is an expression
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break is an expression
        }
    };

    println!("result is: {}", result);

    // loop with labels

    let mut cont = 0;

    'counting_up: loop {
        println!("count = {}", cont);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);

            if remaining == 9 {
                break;
            }

            if cont == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        cont += 1;
    }
    println!("End count: {}", cont);

    // conditional loops

    let mut number = 3;

    while number != 0 {
        println!("Number: {}", number);
        number -=1;
    }

    println!("LIFTOFF!!!");

    // for loop requires less work than a while loop and has a good control over
    let array = [10,20,30,40,50];

    for ele in array {
        println!("The element is: {}", ele);
    }
    
    for ele in 1..10 {
        println!("The element is: {}", ele);
    }


    for ele in (1..10).rev() {
        println!("The element is: {}", ele);
    }
}
