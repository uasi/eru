pub const LC_ALL: libc::c_int = 0;
pub const SIGWINCH: libc::c_int = 28;

extern "C" {
    pub fn setlocale(category: libc::c_int, locale: *const libc::c_char);
}
