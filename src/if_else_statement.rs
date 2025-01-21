pub fn if_else_statement() {
    let a = 3;
    let b = 4;

    if a > b {
        println!("hooray");
    } else {
        println!("foo");
    }
    
}

pub fn iflet_statement() {
    let a = 1;
    let hasil: &str;


    if a >= 10 {
        hasil = "HOORAY";
    } else {
        hasil = "ALAMAKK";
    }

    println!("{}", hasil);
}
//Atau bisa juga kita gabung let dengan if.

#[test]
pub fn ifletgabung_statement() {
    let a = 1;
    let hasil = if a >=10 {
        "HOORAY"
    } else {
        "FUCK"
    };
    println!("{}", hasil);

}

