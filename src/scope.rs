pub fn kopsekop() {
    let anjing = 0;
   // Scope yang ada diluar, dapat digunakan di inner scope, namun tidak sebaliknya. Variabel di
    // dalam inner scope tidak bisa digunakan di luar inner scope. 
    {
        println!("{}", anjing);
        let kucing = 3;
        println!("{}", kucing);
    }
    
}

