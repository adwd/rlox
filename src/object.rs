#[derive(Debug, Clone)]
pub enum Object {
    String(String),
}

pub fn print_object(object: &Object) {
    match object {
        Object::String(s) => print!("{}", s),
    }
}
