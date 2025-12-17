#[cfg(feature = "database")]
pub mod queries;
pub mod migrations;
pub mod pool;

#[cfg(feature = "database")]
pub fn db (){
    println!("Test");
}