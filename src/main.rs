mod facades;

fn main() {
    facades::structs::showcase();
    facades::matching::showcase();

    facades::core::loops::showcase();
    facades::collections::showcase();

    facades::core::datatypes::showcase();
    facades::core::variables::showcase();
    facades::core::shadowing::showcase();
    facades::core::borrowing::showcase();
    facades::core::functions::showcase();
    
    facades::lifetimes::showcase();
    facades::exceptions::showcase();
    facades::core::conditions::showcase();
}
