use crate::canvas::Canvas;

const TOP_FLAG: i32 = 0b1000;
const LEFT_FLAG: i32 = 0b100;
const BOTTOM_FLAG: i32 = 0b10;
const RIGHT_FLAG: i32 = 1;

pub fn get_tile(
    canvas: &Canvas,
    top_opt: Option<bool>, 
    left_opt: Option<bool>,
    bottom_opt: Option<bool>, 
    right_opt: Option<bool>,
) -> char {
    let (tile_flags, bold_flags) = convert_to_flags(top_opt, left_opt, bottom_opt, right_opt);

    let t = is_flag_set(TOP_FLAG, tile_flags);
    let l = is_flag_set(LEFT_FLAG, tile_flags);
    let b = is_flag_set(BOTTOM_FLAG, tile_flags);
    let r = is_flag_set(RIGHT_FLAG, tile_flags);

    let tb = t && is_flag_set(TOP_FLAG, bold_flags);
    let lb = l && is_flag_set(LEFT_FLAG, bold_flags);
    let bb = b && is_flag_set(BOTTOM_FLAG, bold_flags);
    let rb = r && is_flag_set(RIGHT_FLAG, bold_flags);
    
    return if tb && lb && bb && rb {
        '╋'
    } else if tb && lb && bb && r {
        '╉'
    } else if tb && lb && b && rb {
        '╇'
    } else if tb && l && bb && rb {
        '╊'
    } else if t && lb && bb && rb {
        '╈'
    } else if tb && lb && b && r {
        '╃'
    } else if tb && l && bb && r {
        '╂'
    } else if tb && l && b && rb {
        '╄'
    } else if t && lb && bb && r {
        '╅'
    } else if t && lb && b && rb {
        '┿'
    } else if t && l && bb && rb {
        '╆'
    } else if tb && l && b && r {
        '╀'
    } else if t && lb && b && r {
        '┽'
    } else if t && l && bb && r {
        '╁'
    } else if t && l && b && rb {
        '┾'
    } else if t && l && b && r {
        '┼'
    } else if tb && lb && bb {
        '┫'
    } else if tb && lb && b {
        '┩'
    } else if tb && l && bb {
        '┨'
    } else if t && lb && bb {
        '┪'
    } else if tb && l && b {
        '┦'
    } else if t && lb && b {
        '┥'
    } else if t && l && bb {
        '┧'
    } else if t && l && b {
        '┤'
    } else if tb && lb && rb {
        '┻'
    } else if tb && lb && r {
        '┹'
    } else if tb && l && rb {
        '┺'
    } else if t && lb && rb {
        '┷'
    } else if tb && l && r {
        '┸'
    } else if t && lb && r {
        '┵'
    } else if t && l && rb {
        '┶'
    } else if t && l && r {
        '┴'
    } else if tb && bb && rb {
        '┣'
    } else if tb && bb && r {
        '┠'
    } else if tb && b && rb {
        '┡'
    } else if t && bb && rb {
        '┢'
    } else if tb && b && r {
        '┞'
    } else if t && bb && r {
        '┟'
    } else if t && b && rb {
        '┝'
    } else if t && b && r {
        '├'
    } else if lb && bb && rb {
        '┳'
    } else if lb && bb && r {
        '┱'
    } else if lb && b && rb {
        '┯'
    } else if l && bb && rb {
        '┲'
    } else if lb && b && r {
        '┭'
    } else if l && bb && r {
        '┰'
    } else if l && b && rb {
        '┮'
    } else if l && b && r {
        '┬'
    } else if tb && lb {
        '┛'
    } else if tb && l {
        '┚'
    } else if t && lb {
        '┙'
    } else if t && l {
        '┘'
    } else if tb && bb {
        '┃'
    } else if tb && b {
        '╿'
    } else if t && bb {
        '╽'
    } else if t && b {
        '│'
    } else if tb && rb {
        '┗'
    } else if tb && r {
        '┖'
    } else if t && rb {
        '┕'
    } else if t && r {
        '└'
    } else if lb && bb {
        '┓'
    } else if lb && b {
        '┑'
    } else if l && bb {
        '┒'
    } else if l && b {
        '┐'
    } else if lb && rb {
        '━'
    } else if lb && r {
        '╾'
    } else if l && rb {
        '╼'
    } else if l && r {
        '─'
    } else if bb && rb {
        '┏'
    } else if bb && r {
        '┎'
    } else if b && rb {
        '┍'
    } else if b && r {
        '┌'
    } else if tb {
        '╹'
    } else if t {
        '╵'
    } else if lb {
        '╸'
    } else if l {
        '╴'
    } else if bb {
        '╻'
    } else if b {
        '╷'
    } else if rb {
        '╺'
    } else if r { 
        '╶'
    } else {
        canvas.background
    }
}

pub fn is_flag_set(flag: i32, flags: i32) -> bool {
    flags & flag == flag
}

pub fn convert_to_flags(top_opt: Option<bool>, left_opt: Option<bool>, bottom_opt: Option<bool>, right_opt: Option<bool>) -> (i32, i32) {
    let mut tile_flags = 0;
    let mut bold_flags = 0;

    if let Some(bold) = top_opt {
        tile_flags += 1 << 3;
        if bold {
            bold_flags += 1 << 3;
        }
    }
    if let Some(bold) = left_opt {
        tile_flags += 1 << 2;
        if bold {
            bold_flags += 1 << 2;
        }
    }
    if let Some(bold) = bottom_opt {
        tile_flags += 1 << 1;
        if bold {
            bold_flags += 1 << 1;
        }
    }
    if let Some(bold) = right_opt {
        tile_flags += 1;
        if bold {
            bold_flags += 1;
        }
    }
    
    (tile_flags, bold_flags)
}