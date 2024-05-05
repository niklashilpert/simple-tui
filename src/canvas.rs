use std::thread::current;

const HOR_LINE: char = '\u{2500}';
const VER_LINE: char = '\u{2502}';

const CORNER_TL: char = '\u{250C}';
const CORNER_TR: char = '\u{2510}';
const CORNER_BL: char = '\u{2514}';
const CORNER_BR: char = '\u{2518}';


pub struct Canvas {
    pub width: u16,
    pub height: u16,
    pub components: Vec<Component>,
    pub background: char,
}

pub struct Component {
    pub title: String,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    pub fn render(&self) -> String {
        let mut out = self.prepare_background();
        for component in &self.components {
            let tl = (component.x, component.y);
            let tr = (tl.0 + component.width - 1, tl.1);
            let bl = (tl.0, tl.1 + component.height - 1);
            let br = (tr.0, bl.1);

            self.draw_line_vertically((tl.0, tl.1+1), (component.height-2) as usize, VER_LINE, &mut out);
            self.draw_line_vertically((tr.0, tr.1+1), (component.height-2) as usize, VER_LINE, &mut out);            

            self.draw_line_horizontally((tl.0+1, tl.1), (component.width-2) as usize, HOR_LINE, &mut out);
            self.draw_line_horizontally((bl.0+1, bl.1), (component.width-2) as usize, HOR_LINE, &mut out);

            self.set_char_at(tl, CORNER_TL, &mut out);
            self.set_char_at(tr, CORNER_TR, &mut out);
            self.set_char_at(bl, CORNER_BL, &mut out);
            self.set_char_at(br, CORNER_BR, &mut out);   

            self.set_string_horizontally((tl.0 + 2, tl.1), &component.title, &mut out)
        }

        out
    }

    pub fn prepare_counted(&self) -> String {
        let mut out = String::new();
        for _ in 0..self.height {
            let mut count = 0;
            for _ in 0..self.width {
                out.push(char::from_digit(count % 10, 10).unwrap());
                count += 1;
            }
        }
        out
    }

    pub fn prepare_background(&self) -> String {
        let mut out = String::new();
        for _ in 0..self.height {
            for _ in 0..self.width {
                out.push(self.background);
            }
        }
        out
    }

    pub fn set_char_at(&self, pos: (usize, usize), c: char, s: &mut String) {
        let res = self.draw_line_horizontally(pos, 1, c, s);
        res
    }

    pub fn set_string_horizontally(&self, start_pos: (usize, usize), new: &str, s: &mut String) {
        let index: usize = start_pos.1 * self.width as usize + start_pos.0;

        let mut chars_indices = s.char_indices();
        let (first_pos, first_ch) = chars_indices.nth(index).unwrap();
        let mut utf8_len: usize = first_ch.len_utf8();
        for _ in 1..new.chars().count() {
            utf8_len += chars_indices.next().unwrap().1.len_utf8();
        }

        s.replace_range(
            first_pos..first_pos+utf8_len,
            new,
        );
    }

    pub fn draw_line_horizontally(&self, start_pos: (usize, usize), len: usize, c: char, s: &mut String) {
        self.set_string_horizontally(start_pos, &c.to_string().repeat(len), s)
    }

    pub fn draw_line_vertically(&self, start_pos: (usize, usize), len: usize, c: char, s: &mut String) {
         /*
         * The index_offset exists to correct the character positions in the string
         * 
         * Example: "Hello World"
         * 
         * If we want to replace every fifth character, we will replace the 'o' in "Hello" and the 'l' in "World"
         * Let's say, we want to insert a character that takes up three bytes in unicode.
         * Every character after the 'o' will be shifted to the right by two bytes, 
         * since the new character takes up two bytes more than the 'o'.
         * This means that if we want to update the next character (the 'l'), 
         * we will have to change the array two bytes to the right of its original position.
         */
        
        let start_index: usize = start_pos.1 * self.width as usize + start_pos.0;

        let mut chars_indices = s.char_indices();

        let first_elem = chars_indices.nth(start_index).unwrap();
        let mut byte_offset: isize = c.len_utf8() as isize - first_elem.1.len_utf8() as isize;

        let mut to_replace = vec![
            first_elem,
        ];
        
        for _ in 1..len {
            let next = chars_indices.nth(self.width as usize - 1).unwrap();
            to_replace.push(((next.0 as isize + byte_offset) as usize, next.1));
            byte_offset += c.len_utf8() as isize - next.1.len_utf8() as isize;
        }

        for (pos, ch) in to_replace {
            s.replace_range(pos..pos+ch.len_utf8(), &c.to_string());
        }
    }
}
