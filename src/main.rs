pub mod parser;

fn main() {
    let _ = parser::untyped_example();
    let _ = parser::typed_example();
    let result = parser::untyped_error();

    // Handle errors from the parser if any
    match result {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    }

    let result = parser::grab_from_file_untyped();
    match result {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    }
    let result = parser::get_colors();
    match result {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    }
    let result = parser::get_ayas();
    match result {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    }
}
