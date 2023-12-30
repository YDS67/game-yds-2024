use crate::mesh::TextureUV;
use std::collections::HashMap;

pub fn string_to_uv(text: &str) -> Vec<TextureUV> {
    let dictionary: HashMap<char, TextureUV> = HashMap::from([
        ('A', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('B', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('C', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('D', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('E', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('F', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('G', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('H', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('I', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('J', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('K', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('L', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('M', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('N', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('O', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('P', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('Q', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('R', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('S', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('T', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('U', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('V', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('W', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('X', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('Y', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('Z', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('a', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('b', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('c', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('d', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('e', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('f', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('g', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('h', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('i', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('j', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('k', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('l', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('m', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('n', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('o', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('p', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('q', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('r', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('s', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('t', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('u', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('v', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('w', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('x', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('y', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('z', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('1', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('2', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('3', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('4', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('5', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('6', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('7', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('8', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('9', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
        ('0', TextureUV{u1: 0.0, u2: 0.0, v1: 0.0, v2: 0.0}),
    ]);

    let mut coords = Vec::new();

    let letters: Vec<char> = text.chars().collect();

    for letter in letters {

    }

    coords
}