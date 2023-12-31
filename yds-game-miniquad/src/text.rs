use crate::mesh::TextureUV;
use std::collections::HashMap;

pub const WIDTH: f32 = 12.0;
pub const HEIGHT: f32 = 20.0;
pub const HEIGHT_CAP: f32 = 20.0;
const X1: f32 = 3.0;
const Y1: f32 = 2.0;
const Y2: f32 = Y1+HEIGHT_CAP;
const Y3: f32 = Y2+HEIGHT_CAP;
const Y4: f32 = Y3+HEIGHT;
const Y5: f32 = Y4+HEIGHT;
const TEXWIDTH: f32 = 256.0;
const TEXHEIGHT: f32 = 128.0;

pub struct Text {
    pub lines: Vec<String>,
    pub line_width: Vec<f32>,
    pub line_y: Vec<f32>,
    pub font_col: (f32, f32, f32, f32),
    pub act_col: (f32, f32, f32, f32),
    pub act_no: usize,
    pub x0: f32,
    pub y0: f32,
    pub scale: f32,
    pub line_height: f32,
    pub some_active: bool,
}

impl Text {
    pub fn new_from(lines: Vec<&str>) -> Text {
        let x0 = 10.0;
        let y0 = 10.0;
        let scale = 2.0;
        let line_height = HEIGHT*scale;
        let mut lines1 = Vec::new();
        let mut line_width = Vec::new();
        let mut line_y = Vec::new();
        line_y.push(-HEIGHT);
        for l in 0..lines.len() {
            lines1.push(lines[l].to_string());
            let letters: Vec<char> = lines[l].chars().collect();
            line_width.push(letters.len() as f32 * WIDTH * scale);
            line_y.push(y0*scale + (l as f32)*line_height)
        }
        Text {
            lines: lines1,
            line_width,
            line_y,
            font_col: (1.0, 1.0, 0.0, 1.0),
            act_col: (0.8, 0.0, 0.2, 1.0),
            act_no: 0,
            x0,
            y0,
            scale,
            line_height,
            some_active: false,
        }
    }
}

pub fn string_to_uv(text: &str) -> Vec<TextureUV> {
    let dictionary: HashMap<char, TextureUV> = HashMap::from([
        ('A', TextureUV{u1: X1, u2: X1+WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP}),
        ('B', TextureUV{u1: X1+WIDTH, u2: X1+2.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP}),
        ('C', TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP}),
        ('D', TextureUV{u1: X1+3.0*WIDTH, u2: X1+4.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP}),
        ('E', TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP}),
        ('F', TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP}),
        ('G', TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP}),
        ('H', TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP}),
        ('I', TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP}),
        ('J', TextureUV{u1: X1+9.0*WIDTH, u2: X1+10.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP}),
        ('K', TextureUV{u1: X1+10.0*WIDTH, u2: X1+11.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP}),
        ('L', TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP}),
        ('M', TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP}),
        ('N', TextureUV{u1: X1+0.0*WIDTH, u2: X1+1.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP}),
        ('O', TextureUV{u1: X1+1.0*WIDTH, u2: X1+2.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP}),
        ('P', TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP}),
        ('Q', TextureUV{u1: X1+3.0*WIDTH, u2: X1+4.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP}),
        ('R', TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP}),
        ('S', TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP}),
        ('T', TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP}),
        ('U', TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP}),
        ('V', TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP}),
        ('W', TextureUV{u1: X1+9.0*WIDTH, u2: X1+10.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP}),
        ('X', TextureUV{u1: X1+10.0*WIDTH, u2: X1+11.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP}),
        ('Y', TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP}),
        ('Z', TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP}),
        ('a', TextureUV{u1: X1+0.0*WIDTH, u2: X1+1.0*WIDTH, v1: Y3, v2: Y3+HEIGHT}),
        ('b', TextureUV{u1: X1+1.0*WIDTH, u2: X1+2.0*WIDTH, v1: Y3, v2: Y3+HEIGHT}),
        ('c', TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y3, v2: Y3+HEIGHT}),
        ('d', TextureUV{u1: X1+3.0*WIDTH, u2: X1+4.0*WIDTH, v1: Y3, v2: Y3+HEIGHT}),
        ('e', TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y3, v2: Y3+HEIGHT}),
        ('f', TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y3, v2: Y3+HEIGHT}),
        ('g', TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y3, v2: Y3+HEIGHT}),
        ('h', TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y3, v2: Y3+HEIGHT}),
        ('i', TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y3, v2: Y3+HEIGHT}),
        ('j', TextureUV{u1: X1+9.0*WIDTH, u2: X1+10.0*WIDTH, v1: Y3, v2: Y3+HEIGHT}),
        ('k', TextureUV{u1: X1+10.0*WIDTH, u2: X1+11.0*WIDTH, v1: Y3, v2: Y3+HEIGHT}),
        ('l', TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y3, v2: Y3+HEIGHT}),
        ('m', TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y3, v2: Y3+HEIGHT}),
        ('n', TextureUV{u1: X1+0.0*WIDTH, u2: X1+1.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('o', TextureUV{u1: X1+1.0*WIDTH, u2: X1+2.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('p', TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('q', TextureUV{u1: X1+3.0*WIDTH, u2: X1+4.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('r', TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('s', TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('t', TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('u', TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('v', TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('w', TextureUV{u1: X1+9.0*WIDTH, u2: X1+10.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('x', TextureUV{u1: X1+10.0*WIDTH, u2: X1+11.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('y', TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('z', TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('*', TextureUV{u1: X1+13.0*WIDTH, u2: X1+14.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('“', TextureUV{u1: X1+14.0*WIDTH, u2: X1+15.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('”', TextureUV{u1: X1+15.0*WIDTH, u2: X1+16.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('!', TextureUV{u1: X1+16.0*WIDTH, u2: X1+17.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('’', TextureUV{u1: X1+17.0*WIDTH, u2: X1+18.0*WIDTH, v1: Y4, v2: Y4+HEIGHT}),
        ('1', TextureUV{u1: X1+0.0*WIDTH, u2: X1+1.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        ('2', TextureUV{u1: X1+1.0*WIDTH, u2: X1+2.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        ('3', TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        ('4', TextureUV{u1: X1+3.0*WIDTH, u2: X1+4.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        ('5', TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        ('6', TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        ('7', TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        ('8', TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        ('9', TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        ('0', TextureUV{u1: X1+9.0*WIDTH, u2: X1+10.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        (' ', TextureUV{u1: X1+10.0*WIDTH, u2: X1+11.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        ('.', TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        (':', TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        (',', TextureUV{u1: X1+13.0*WIDTH, u2: X1+14.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        ('?', TextureUV{u1: X1+14.0*WIDTH, u2: X1+15.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        ('(', TextureUV{u1: X1+15.0*WIDTH, u2: X1+16.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        (')', TextureUV{u1: X1+16.0*WIDTH, u2: X1+17.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
        (';', TextureUV{u1: X1+17.0*WIDTH, u2: X1+18.0*WIDTH, v1: Y5, v2: Y5+HEIGHT}),
    ]);

    let mut coords = Vec::new();

    let letters: Vec<char> = text.chars().collect();

    for letter in letters {
        coords.push(*dictionary.get(&letter).unwrap())
    }

    for c in 0..coords.len() {
        coords[c].normalize(TEXWIDTH, TEXHEIGHT);
    }

    coords
}