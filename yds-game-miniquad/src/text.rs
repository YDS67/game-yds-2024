use crate::mesh::TextureUV;
use std::collections::HashMap;

pub const WIDTH: f32 = 12.0;
pub const HEIGHT: f32 = 20.0;
const X1: f32 = 3.0;
const Y1: f32 = 2.0;
const Y2: f32 = 23.0;
const Y3: f32 = 56.0;
const Y4: f32 = Y3+HEIGHT;
const Y5: f32 = 111.0;

pub fn string_to_uv(text: &str) -> Vec<TextureUV> {
    let dictionary: HashMap<char, TextureUV> = HashMap::from([
        ('A', TextureUV{u1: X1, u2: X1+WIDTH, v1: Y1, v2: Y1+HEIGHT}),
        ('B', TextureUV{u1: X1+WIDTH, u2: X1+2.0*WIDTH, v1: Y1, v2: Y1+HEIGHT}),
        ('C', TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y1, v2: Y1+HEIGHT}),
        ('D', TextureUV{u1: X1+3.0*WIDTH, u2: X1+4.0*WIDTH, v1: Y1, v2: Y1+HEIGHT}),
        ('E', TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y1, v2: Y1+HEIGHT}),
        ('F', TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y1, v2: Y1+HEIGHT}),
        ('G', TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y1, v2: Y1+HEIGHT}),
        ('H', TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y1, v2: Y1+HEIGHT}),
        ('I', TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y1, v2: Y1+HEIGHT}),
        ('J', TextureUV{u1: X1+9.0*WIDTH, u2: X1+10.0*WIDTH, v1: Y1, v2: Y1+HEIGHT}),
        ('K', TextureUV{u1: X1+10.0*WIDTH, u2: X1+11.0*WIDTH, v1: Y1, v2: Y1+HEIGHT}),
        ('L', TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y1, v2: Y1+HEIGHT}),
        ('M', TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y1, v2: Y1+HEIGHT}),
        ('N', TextureUV{u1: X1+0.0*WIDTH, u2: X1+1.0*WIDTH, v1: Y2, v2: Y2+HEIGHT}),
        ('O', TextureUV{u1: X1+1.0*WIDTH, u2: X1+2.0*WIDTH, v1: Y2, v2: Y2+HEIGHT}),
        ('P', TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y2, v2: Y2+HEIGHT}),
        ('Q', TextureUV{u1: X1+3.0*WIDTH, u2: X1+4.0*WIDTH, v1: Y2, v2: Y2+HEIGHT}),
        ('R', TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y2, v2: Y2+HEIGHT}),
        ('S', TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y2, v2: Y2+HEIGHT}),
        ('T', TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y2, v2: Y2+HEIGHT}),
        ('U', TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y2, v2: Y2+HEIGHT}),
        ('V', TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y2, v2: Y2+HEIGHT}),
        ('W', TextureUV{u1: X1+9.0*WIDTH, u2: X1+10.0*WIDTH, v1: Y2, v2: Y2+HEIGHT}),
        ('X', TextureUV{u1: X1+10.0*WIDTH, u2: X1+11.0*WIDTH, v1: Y2, v2: Y2+HEIGHT}),
        ('Y', TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y2, v2: Y2+HEIGHT}),
        ('Z', TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y2, v2: Y2+HEIGHT}),
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
    ]);

    let mut coords = Vec::new();

    let letters: Vec<char> = text.chars().collect();

    for letter in letters {
        coords.push(*dictionary.get(&letter).unwrap())
    }

    coords
}