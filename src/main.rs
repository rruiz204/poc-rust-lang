use poc_rust_lang::facades::variables;
use poc_rust_lang::facades::datatypes;
use poc_rust_lang::facades::functions;
use poc_rust_lang::facades::controlled;
use poc_rust_lang::facades::matching;

fn main() {
    variables::facade();
    datatypes::facade();
    functions::facade();
    controlled::facade();
    matching::facade();
}
