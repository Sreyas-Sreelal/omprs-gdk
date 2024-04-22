use omprs_codegen::native;
native!(Print,message: str);

pub fn load_functions() {
    load_function!(Print);
}
