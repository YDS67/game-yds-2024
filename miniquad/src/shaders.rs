use miniquad::*;
use glam;

pub const VERTEX: &str = r#"#version 330 core
in vec3 pos;
in vec2 uv;

uniform mat4 mvp;
uniform vec3 playerpos;

out vec2 texcoord;
out vec4 cols;

float col1, d;
vec3 dir;

void main() {
    dir = pos - playerpos;
    d = length(dir);

    gl_Position = mvp * vec4(pos, 1.0);

    col1 = 1.0/(1.0+(d/5.0)*(d/5.0));
    cols = vec4(col1,col1,col1,1.0);
    
    texcoord = uv;
}"#;

pub const FRAGMENT: &str = r#"#version 330 core
in vec2 texcoord;
in vec4 cols;

out vec4 FragColor;

uniform sampler2D tex;

void main() {
    FragColor = vec4(texture(tex, texcoord).xyz * cols.xyz, 1.0);
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
