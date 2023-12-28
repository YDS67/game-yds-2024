use miniquad::*;
use glam;

pub const VERTEX: &str = r#"#version 330
attribute vec3 pos;
attribute vec2 uv;
attribute vec4 col;

uniform mat4 mvp;
uniform vec3 playerpos;

varying vec2 texcoord;
varying vec4 cols;

float col1, l;
vec3 dir;

void main() {
    dir = pos - playerpos;
    l = length(dir);

    gl_Position = mvp * vec4(pos, 1.0);

    col1 = 1.0/(1.0+(l/10.0)*(l/10.0));
    cols = vec4(col1,0.8*col1,0.7*col1,1.0);
    
    texcoord = uv;
}"#;

pub const FRAGMENT: &str = r#"#version 330
varying vec2 texcoord;
varying vec4 cols;

uniform sampler2D tex;

void main() {
    gl_FragColor = vec4(texture(tex, texcoord).xyz * cols.xyz, 1.0);
}"#;

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("mvp", UniformType::Mat4),
                UniformDesc::new("playerpos", UniformType::Float3),
            ],
        },
    }
}

#[repr(C)]
pub struct Uniforms {
    pub mvp: glam::Mat4,
    pub playerpos: (f32, f32, f32),
}
