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
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    pub fn render(&self) -> String {
        let mut out = self.prepare_empty();
        for component in &self.components {
            let tl = (component.x, component.y);
            let tr = (tl.0 + component.width, tl.1);
            let bl = (tl.0, tl.1 + component.height);
            let br = (tr.0, bl.1);


            self.set_chars_vertically(tl, component.height as usize, VER_LINE, &mut out);

            self.set_chars_horizontally(tl, component.width as usize, HOR_LINE, &mut out);
            self.set_chars_horizontally(bl, component.width as usize, HOR_LINE, &mut out);

            self.set_chars_vertically(tr, component.height as usize, VER_LINE, &mut out);            

            self.set_char_at(tl, CORNER_TL, &mut out);
            self.set_char_at(tr, CORNER_TR, &mut out);
            self.set_char_at(bl, CORNER_BL, &mut out);
            self.set_char_at(br, CORNER_BR, &mut out);   
        }

        out
    }

    pub fn prepare_empty(&self) -> String {
        let mut out = String::new();

        for _ in 0..self.height {
            for _ in 0..self.width {
                out.push(self.background);
            }
        }
        out
    }

    pub fn set_char_at(&self, pos: (usize, usize), c: char, s: &mut String) {
        let res = self.set_chars_horizontally(pos, 1, c, s);
        res
    }

    pub fn set_chars_horizontally(&self, start_pos: (usize, usize), len: usize, c: char, s: &mut String) {
        let index: usize = start_pos.1 * self.width as usize + start_pos.0;
        if index >= s.len() {
            return
        }

        let mut chars_indices = s.char_indices();
        let (first_pos, first_ch) = chars_indices.nth(index).unwrap();
        let mut utf8_len: usize = first_ch.len_utf8();
        for _ in 1..len {
            utf8_len += chars_indices.next().unwrap().1.len_utf8();
        }

        s.replace_range(
            first_pos..first_pos+utf8_len,
            &c.to_string().repeat(len),
        );
    }

    pub fn set_chars_vertically(&self, start_pos: (usize, usize), len: usize, c: char, s: &mut String) {
        let index: usize = start_pos.1 * self.width as usize + start_pos.0;

        let mut chars_indices = s.char_indices();
        let mut chars_indices_to_replace = vec![
            chars_indices.nth(index).unwrap(),
        ];
        
        for _ in 1..len {
            let next_result = chars_indices.nth(self.width as usize + 1);
            if let Some(next) = next_result {
                chars_indices_to_replace.push(next);
            }
        }

        for (pos, ch) in chars_indices_to_replace {
            s.replace_range(pos..pos+ch.len_utf8(), &c.to_string());
        }
    }
}
