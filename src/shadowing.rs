pub fn shadowing() {
    let foo = "kenapa";
    println!("{}", foo);

    let foo = "10";
    println!("{}", foo);
}
