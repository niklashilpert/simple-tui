pub enum TitleAlign {
    Left,
    Right,
    Center,
}

pub trait Container<'a> {
    fn get_bounds(&self) -> (u16, u16, u16, u16);
    fn is_bold(&self) -> bool;
    fn get_title(&self) -> String;
    fn get_title_alignment(&self) -> TitleAlign;
}

pub struct TextInput {
    title: String,
    pos: (u16, u16),
    prefix_str: String,
    prefix_len: u16,
    input_len: u16,
    padding: (u16, u16, u16, u16),
}

impl<'a> Container<'a> for TextInput {
    fn get_bounds(&self) -> (u16, u16, u16, u16) {
        (
            self.pos.0,
            self.pos.1,
            self.padding.1 + self.prefix_len + self.input_len + self.padding.3,
            self.padding.0 + 1 + self.padding.2,
        )
    }

    fn is_bold(&self) -> bool {
        false
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_title_alignment(&self) -> TitleAlign {
        TitleAlign::Center
    }

}