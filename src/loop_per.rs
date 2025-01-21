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

pub fn loop2() {
    let mut nomor = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if nomor > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", nomor, i, nomor * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        nomor += 1;
    }
}

