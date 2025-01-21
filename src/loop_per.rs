pub fn per_loop() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 10{
            break;
        } else if counter % 2 == 0 {
            continue;
        }
    println!("Counter: {}", counter);

    }
}

