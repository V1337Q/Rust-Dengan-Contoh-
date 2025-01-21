// Tempat untuk menjalankan code yang ada di /src. Include file dengan menggunakan
// mod (namafile);
//
// jalankan kode program dengan command = cargo test (nama test function) -- --exact --show-output
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
mod ownership_move;
mod if_expression;
mod loop_per;
mod while_loop; 
mod for_loop;
mod for_loop_array;
mod range;
mod function_call;
mod parameter;
mod rekursi;
mod struct_1;
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

#[test]
fn if_let() {
    if_else_statement::iflet_statement();
}

// If expression. If juga bisa dinyatakan sebagai suatu variable.
#[test]
fn if_expression() {
    if_expression::ifletgabung_statement();
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

#[test]
fn move_ownership() {
    ownership_move::ownershipmovement();
}

//test loop 
#[test]
fn loop_1() {
    loop_per::per_loop();
}

#[test]
fn loop_2() {
    loop_per::loop2();
}

//test while loop
#[test]
fn while_loop() {
    while_loop::while_loop();
}
#[test]
fn array_loop() {
    while_loop::while_array();
}

//test for loop 
#[test]
fn for_loop() {
    for_loop::for_loop();
}

//for loop untuk array
#[test]
fn for_loop_array() {
    for_loop_array::for_array();
}

//test range
#[test]
fn range() {
    range::range();
}

//inclusive range
#[test]
fn inclusive_range() {
    range::range_inclusive();
}

//test function calling
#[test]
fn function_call() {
    function_call::panggil_katakan_keju(); 
}

//test parameter 
#[test]
fn parameter() {
    parameter::test_parameter();
}

//test konsep rekursi
#[test]
fn rekursi() {
    rekursi::coba_rekursi();
}

//Aplikasi rekursi pada rumus faktorial
#[test]
fn faktorial() {
    rekursi::faktorekur();
}

//Struct 

#[test]
fn struct_1() {
    struct_1::cobaidentitas();
}

//test konsep enumeration
#[test]
fn enumeration() {
    enumeration::payment_test();
}


