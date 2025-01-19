pub enum Bank {
    Nomor(String, String),
    NamaBank(String),
    NamaUser(String),
}

pub impl Bank {
    pub fn payment(&self, jumlah: u32) { //Self ini mengacu kepada imp Bank. jadi  match self sama saja
        //dengan match Bank.
        match self {
            Bank::Nomor(number, bank) => { //Jadi, nilai yang ada di field enum Bank akan diisi
                //dengan nilai baru yang di assign disini. 
                println!("Membayar dengan nomor: {}, bank {} sejumlah: {}", number, bank, jumlah);
            }
            Bank::NamaUser(number) => {
                println!("Membayar dengan nomor: {}, sejumlah: {}", number, jumlah);
            }
            Bank::NamaBank(number) => {
                println!("Membayar dengan nomor: {} sejumlah: {}",  number, jumlah);
            }
        }
    }
}


pub fn payment_test() {
    let data_bank = Bank::Nomor(String::from("BJB"), String::from("12223"));
    data_bank.payment(210000);
    let data_bank = Bank::NamaBank(String::from("12223"));
    data_bank.payment(240000);
    let data_bank = Bank::NamaUser(String::from("12223"));
    data_bank.payment(230000);
}

