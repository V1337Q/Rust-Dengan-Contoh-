pub fn range() {

    let range = 0..5;
    println!("Mulai (inclusive): {}", range.start);
    println!("Akhir (exvlusive/exlude): {}", range.end);
    
    for x in range {
        println!("{}", x * 2);
    }
}

//inclusive range
//
pub fn range_inclusive() {

    let range = 0..=5;
    println!("Mulai (inclusive): {}", range.start());
    println!("Akhir (inclusive): {}", range.end());//Start dan End nya merupakan function,
    //jadi pakai (). Bukan atribute.
    
    for x in range {
        println!("{}", x * 2);
    }
}

