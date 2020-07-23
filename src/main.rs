use std::ffi::CString;
use std::io::{self, BufRead};

mod fsm;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let cs = CString::new(line?).unwrap();
        if unsafe { fsm::fsm_main(cs.as_ptr()) } > 0 {
            println!("Matched!");
        } else {
            println!("Didn't match!");
        }
    }
    Ok(())
}
