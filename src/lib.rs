extern crate postgres;

use postgres::{Connection, TlsMode};
use std::os::raw::c_char;
use std::ffi::{CStr};

struct Count {
    id: i64,
}
//
// #[no_mangle]
// pub extern fn index() {
//     println!("check code");
// }
#[no_mangle]
pub extern fn rust_count( c_string: *const c_char ) {
    let mut total_all = 0;
    let mut total_filter = 0;
    let conn = Connection::connect("postgresql://auth:password@localhost:5432/db_verification_service", TlsMode::None).unwrap();
    let s = unsafe { CStr::from_ptr(c_string).to_string_lossy().into_owned() };
    for row in &conn.query(&s, &[]).unwrap() {
        let count = Count {
            id: row.get(0)
        };
	total_all = count.id ;
        // println!("{}", count.id);
    }
    for row in &conn.query(&s, &[]).unwrap() {
        let count = Count {
            id: row.get(0)
        };
	total_filter = count.id ;
        // println!("{}", count.id);
    }
    println!("[ {:?}, {:?} ]", total_all ,total_filter );
}
