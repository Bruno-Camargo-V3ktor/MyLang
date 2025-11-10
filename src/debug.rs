use std::sync::Mutex;

static HAD_ERROR: Mutex<bool> = Mutex::new(false);
pub struct MyDebug {}

impl MyDebug {
    fn set_had_error(value: bool) {
        if let Ok(mut val) = HAD_ERROR.lock() {
            *val = value;
        }
    }

    fn report(line: usize, source: &str, message: &str) {
        Self::set_had_error(true);
        eprintln!("error: line: {line}, where: {source}, message: {message}");
    }

    pub fn error(line: usize, message: &str) {
        Self::report(line, "", message);
    }
}
