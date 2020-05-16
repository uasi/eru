use libc;
use ncurses as nc;
use std::ffi::CString;

use libc_aux;
use screen_data::ScreenData;
use window::{Rect, Window};
use window::{Gutter, ListView, MiniBuf, StatusLine};

pub struct Screen {
    gutter: Window,
    list_view: Window,
    mini_buf: Window,
    status_line: Window,
}

impl Screen {
    pub fn new() -> Self {
        let layout = Layout::new();
        let gutter = Window::new(Gutter, layout.gutter_rect);
        let list_view = Window::new(ListView, layout.list_view_rect);
        let mini_buf = Window::new(MiniBuf, layout.mini_buf_rect);
        let status_line = Window::new(StatusLine, layout.status_line_rect);
        Screen {
            gutter: gutter,
            list_view: list_view,
            mini_buf: mini_buf,
            status_line: status_line,
        }
    }

    pub fn update(&self, sd: ScreenData) {
        for win in [&self.gutter, &self.list_view, &self.mini_buf, &self.status_line].iter() {
            win.clear();
            win.draw(&sd);
            win.noutrefresh();
        }
        MiniBuf::set_cursor(&self.mini_buf, sd.cursor_index as i32);
        nc::doupdate();
    }

    pub fn list_view_height(&self) -> usize {
        self.list_view.rect().height as usize
    }

    pub fn resize(&mut self) {
        // Resize stdscr.
        nc::endwin();
        nc::initscr();

        let layout = Layout::new();
        self.gutter.resize(layout.gutter_rect);
        self.list_view.resize(layout.list_view_rect);
        self.mini_buf.resize(layout.mini_buf_rect);
        self.status_line.resize(layout.status_line_rect);
    }
}

struct Layout {
    gutter_rect: Rect,
    list_view_rect: Rect,
    mini_buf_rect: Rect,
    status_line_rect: Rect,
}

impl Layout {
    fn new() -> Layout {
        let mut max_y = 0i32;
        let mut max_x = 0i32;
        unsafe { nc::getmaxyx(nc::stdscr, &mut max_y, &mut max_x) };

        let gutter_width = 2;
        let mini_buf_height = 1;
        let status_line_height = 1;

        let gutter_rect = Rect {
            height: max_y - mini_buf_height - status_line_height,
            width: gutter_width,
            y: mini_buf_height + status_line_height,
            x: 0,
        };

        let list_view_rect = Rect {
            height: max_y - mini_buf_height - status_line_height,
            width: max_x - gutter_width,
            y: mini_buf_height + status_line_height,
            x: gutter_width,
        };

        let mini_buf_rect = Rect {
            height: mini_buf_height,
            width: max_x,
            y: 0,
            x: 0,
        };

        let status_line_rect = Rect {
            height: status_line_height,
            width: max_x - gutter_width,
            y: mini_buf_height,
            x: gutter_width,
        };

        Layout {
            gutter_rect: gutter_rect,
            list_view_rect: list_view_rect,
            mini_buf_rect: mini_buf_rect,
            status_line_rect: status_line_rect,
        }
    }
}

pub fn initialize() {
    let s = CString::new("").unwrap();
    unsafe {
        libc_aux::setlocale(libc_aux::LC_ALL, s.as_ptr());
        libc::dup2(1, 3);
        libc::dup2(2, 1);
    }
    nc::initscr();
    nc::noecho();
    nc::raw();
}

pub fn finalize() {
    nc::endwin();
    unsafe {
        libc::dup2(3, 1);
    }
}
