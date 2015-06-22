use ncurses as nc;
use std::cmp;

use screen_data::ScreenData;

pub struct Window {
    rect: Rect,
    win: nc::WINDOW,
    win_impl: Box<WindowImpl + Send>
}

impl Window {
    pub fn new<WI>(win_impl: WI, r: Rect) -> Window
        where WI: WindowImpl + Send + 'static
    {
        let win = nc::newwin(r.height, r.width, r.y, r.x);
        nc::leaveok(win, true);
        Window {
            rect: r,
            win: win,
            win_impl: Box::new(win_impl)
        }
    }

    pub fn draw(&self, sd: &ScreenData) {
        self.win_impl.draw(self.win, self.rect, sd);
    }

    pub fn clear(&self) {
        nc::wclear(self.win);
    }

    pub fn noutrefresh(&self) {
        nc::wnoutrefresh(self.win);
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }
}

pub trait WindowImpl {
    fn draw(&self, win: nc::WINDOW, r: Rect, sd: &ScreenData);
}

pub struct Gutter;

impl WindowImpl for Gutter {
    fn draw(&self, win: nc::WINDOW, r: Rect, sd: &ScreenData) {
        // TODO: shift
        if sd.items.len() > 0 {
            let i = cmp::min(sd.highlighted_row as i32, r.height);
            nc::mvwaddstr(win, i as i32, 0, ">");
        }
        for row in sd.selected_rows.iter() {
            let i = cmp::min(*row as i32, r.height);
            nc::mvwaddstr(win, i as i32, 1, ">");
        }
    }
}

pub struct MiniBuf;

impl WindowImpl for MiniBuf {
    fn draw(&self, win: nc::WINDOW, _r: Rect, sd: &ScreenData) {
        nc::mvwaddstr(win, 0, 0, &*sd.query_string);
    }
}

impl MiniBuf {
    pub fn set_cursor(win: &Window, cursor_index: i32) {
        let mut beg_y = 0i32;
        let mut beg_x = 0i32;
        nc::getbegyx(win.win, &mut beg_y, &mut beg_x);
        let mut cur_y = 0i32;
        let mut cur_x = 0i32;
        nc::getyx(win.win, &mut cur_y, &mut cur_x);
        let x_offset = 0; // use someday
        let mut scr_cur_y = beg_y + cur_y;
        let mut scr_cur_x = beg_x + x_offset + cursor_index;
        nc::setsyx(&mut scr_cur_y, &mut scr_cur_x);
    }
}

pub struct ListView;

// TODO: shift
impl WindowImpl for ListView {
    fn draw(&self, win: nc::WINDOW, r: Rect, sd: &ScreenData) {
        let num_lines = cmp::min(sd.items.len(), r.height as usize);
        for (y, item) in sd.items.iter().take(num_lines).enumerate() {
            nc::mvwaddstr(win, y as i32, 0, item.as_ref()); // TODO: truncate line
        }
    }
}

pub struct StatusLine;

impl WindowImpl for StatusLine {
    fn draw(&self, win: nc::WINDOW, _r: Rect, sd: &ScreenData) {
        let s = format!("{}/{}", sd.item_list_len, sd.total_lines);
        nc::mvwaddstr(win, 0, 0, &s);
    }
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub height: i32,
    pub width: i32,
    pub y: i32,
    pub x: i32,
}
