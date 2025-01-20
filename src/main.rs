//Tempat untuk menjalankan code yang ada di /src. Include file dengan menggunakan
//mod (namafile);
//
//jalankan kode program dengan command = cargo test (nama test function) -- --exact --show-output
mod variable;
mod merubah_tipedata;
mod operator;
mod tuple;
mod shadowing;
mod comparator;
mod if_else_statement;
mod array_indexing;
mod scope;
mod enumeration;
mod boolean;
mod array;
mod stringslice;
mod two_d_array;
mod konstanta;
mod tipe_string;
mod stack_heap;


//test variable
#[test]
fn test_variable() {
    variable::variable();
}

//test merubah tipe data
#[test]
fn test_merubah_tipedata() {
    merubah_tipedata::merubah_tipedata();
}

//test operator
#[test]
fn operator() {
    operator::operator_simpel();
}

//test operasi boolean
#[test]
fn boolean() {
    boolean::operasibool();
}

//test tuple, dan cara mengakses tuple
#[test]
fn test_tuple() {
    tuple::tuple();
}

//test array
#[test]
fn array() {
    array::arayy();
}

#[test]
fn two_d_array() {
    two_d_array::twodarray();
}

//test konsep shadowing
#[test]
fn shadowing() {
    shadowing::shadowing();
}

//test comparator
#[test]
fn comparator() {
    comparator::comparator();
}

//test if else statement 
#[test]
fn if_else_statement() {
    if_else_statement::if_else_statement();
}

//test array indexing 
#[test]
fn array_indexing() {
    array_indexing::arayy_indexing();
}

//test Stringslice
#[test]
fn str() {
    stringslice::stringslice();
}

//test Tipe String
#[test]
fn tipe_string() {
    tipe_string::tipestring();
}

//test konsep scope
#[test]
fn scope() {
    scope::kopsekop();
}

//test Const
#[test]
fn konstanta() {
    konstanta::konstan();
}

//Konsep Ownership, dan alokasi data di Rust
#[test]
fn copy_data() {
    stack_heap::datacopy();
}

//test konsep enumeration
#[test]
fn enumeration() {
    enumeration::payment_test();
}


