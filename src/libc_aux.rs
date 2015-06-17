use libc;

pub const LC_ALL: libc::c_int = 0;

extern {
    pub fn setlocale(category: libc::c_int, locale: *const libc::c_char);
}
