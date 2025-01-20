pub fn tipestring() {
    let mut name= String::from("Budi Rahmat");
    name.push_str(" Susanto");
    println!("{}", name);

    let kai = name.replace("Budi", "Kai");
    println!("{}", kai);

    assert_eq!("Kai Rahmat Susanto", kai)
}

