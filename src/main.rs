use crate::facades::variables;
use crate::facades::datatypes;
use crate::facades::functions;
use crate::facades::controlled;
use crate::facades::matching;
use crate::facades::structs;
use crate::facades::vectors;
use crate::facades::enums;
use crate::facades::exceptions;

pub mod facades;

fn main() {
    variables::facade();
    datatypes::facade();
    functions::facade();
    controlled::facade();
    matching::facade();
    structs::facade();
    enums::facade();
    vectors::facade();
    exceptions::facade();
}
