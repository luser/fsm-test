#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn fsm_main(mut s: *const libc::c_char) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_int = 0;
    p = s;
    let fresh0 = p;
    p = p.offset(1);
    c = *fresh0 as libc::c_uchar as libc::c_int;
    if c == '\u{0}' as i32 { return -(1 as libc::c_int) }
    if c == 'a' as i32 {
        loop  {
            /* e.g. "a" */
            let fresh3 = p; /* "ab*c|d+" */
            p = p.offset(1); /* "ab*c|d+" */
            c = *fresh3 as libc::c_uchar as libc::c_int;
            if c == '\u{0}' as i32 { return -(1 as libc::c_int) }
            if !(c == 'b' as i32) { break ; }
        }
        if c != 'c' as i32 { return -(1 as libc::c_int) }
        let fresh4 = p;
        p = p.offset(1);
        c = *fresh4 as libc::c_uchar as libc::c_int;
        if c == '\u{0}' as i32 { return 0x1 as libc::c_int }
        return -(1 as libc::c_int)
    } else if c == 'd' as i32 {
        loop 
             /* e.g. "d" */
             {
            let fresh5 = p;
            p = p.offset(1);
            c = *fresh5 as libc::c_uchar as libc::c_int;
            if c == '\u{0}' as i32 { return 0x1 as libc::c_int }
            if c != 'd' as i32 { return -(1 as libc::c_int) }
        }
    } else {
        if c == 'x' as i32 {
            /* e.g. "x" */
            let fresh6 = p;
            p = p.offset(1);
            c = *fresh6 as libc::c_uchar as libc::c_int;
            if c == '\u{0}' as i32 { return -(1 as libc::c_int) }
            if c != 'y' as i32 { return -(1 as libc::c_int) }
        } else if c != 'y' as i32 { return -(1 as libc::c_int) }
        /* e.g. "y" */
        let fresh1 = p; /* "x?yz" */
        p = p.offset(1);
        c = *fresh1 as libc::c_uchar as libc::c_int;
        if c == '\u{0}' as i32 { return -(1 as libc::c_int) }
        if c != 'z' as i32 { return -(1 as libc::c_int) }
        let fresh2 = p;
        p = p.offset(1);
        c = *fresh2 as libc::c_uchar as libc::c_int;
        if c == '\u{0}' as i32 { return 0x2 as libc::c_int }
        return -(1 as libc::c_int)
    };
}
