use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // My idea is to find all the files in ./plugins and then manipulate them here
    // so that the main function can access them all without knowing beforehand
    // what plugins exist and what don't. This should be as smooth as possible.
    // Maybe something with renaming the functions, need to learn more what we
    // can do in this file.
}
