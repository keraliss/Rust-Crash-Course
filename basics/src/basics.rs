pub fn run() {
    let c = add(5, 15);
    println!("c = {c}");

    let a = 99;
    if a > 98 {
        println!("big");
    } else {
        println!("small");
    }
    let b = 80;
    if b > 100 {
        if b > 200 {
            println!("huge number");
        } else {
            println!("small number");
        }
    } else {
        println!("very small number");
    }

    let d = 150;
    if d > 30 {
        println!("big number");
    } else if d > 20 {
        println!("medium number");
    } else {
        println!("small number");
    }

    let mut e = 0;
    let mut f = 0;
    loop {
        if e == 5 {
            break;
        }
        println!("loop e = {e}");
        e = e + 1;
    }

    while f < 10 {
        println!(" while f = {f}");
        f = f + 1;
    }
}


fn add(x: i32, y: i32) -> i32 {
    x + y
}
