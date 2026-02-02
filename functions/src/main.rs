fn main() {
    let y = {
        let x = five();
        plus_one(x)
    };

    println!("the value here is {y}");
    another_function(5);
    looper();
    forloop();
}



fn another_function(x: i32){
    println!("We got number {x} as parameter");
}

fn five () -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}


fn looper(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!{"The result is {result}"};
}

fn forloop(){
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}