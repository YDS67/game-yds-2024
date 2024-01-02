use miniquad::*;
use glam;

pub const VERTEX_MAIN: &str = r#"#version 330 core
in vec3 pos;
in vec2 uv;

uniform mat4 mvp;
uniform vec3 playerpos;
uniform float lightdist;

out vec2 texcoord;
out vec4 cols;

float col1, d;
vec3 dir;

void main() {
    dir = pos - playerpos;
    d = length(dir);

    gl_Position = mvp * vec4(pos, 1.0);

    col1 = 1.0/(1.0+(d/lightdist)*(d/lightdist));
    cols = vec4(col1,col1,col1,1.0);
    
    texcoord = uv;
}"#;

pub const FRAGMENT_MAIN: &str = r#"#version 330 core
in vec2 texcoord;
in vec4 cols;

out vec4 FragColor;

uniform sampler2D tex;

void main() {
    FragColor = vec4(texture(tex, texcoord).xyz * cols.xyz, 1.0);
}"#;

pub const VERTEX_TEXT: &str = r#"#version 330 core
in vec3 pos;
in vec2 uv;

out vec2 texcoord;
out vec2 screenpos;

void main() {
    gl_Position = vec4((pos.x-0.5)*2.0, (0.5-pos.y)*2.0, 0.0, 1.0);
    texcoord = uv;
    screenpos = gl_Position.xy;
}"#;

pub const FRAGMENT_TEXT: &str = r#"#version 330 core
in vec2 texcoord;
in vec2 screenpos;

out vec4 FragColor;

uniform sampler2D tex;
uniform vec4 fontcolor;
uniform vec4 actcolor;
uniform vec2 activeline;

vec4 col;

void main() {
    col = texture(tex, texcoord);

    if (col.x+col.y+col.z > 2.99) {
        discard;
    } else {
        if (screenpos.y <= (0.5-activeline.x)*2.0 && screenpos.y >= (0.5-activeline.y)*2.0) {
            FragColor = actcolor;
        } else {
            if (screenpos.x <= 0.1 && screenpos.x >= -0.1) {
                FragColor = actcolor;
            } else {
                FragColor = fontcolor;
            }   
        }
        
    }
}"#;

pub fn meta_main() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("mvp", UniformType::Mat4),
                UniformDesc::new("playerpos", UniformType::Float3),
                UniformDesc::new("lightdist", UniformType::Float1),
            ],
        },
    }
}

pub fn meta_text() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("fontcolor", UniformType::Float4),
                UniformDesc::new("actcolor", UniformType::Float4),
                UniformDesc::new("activeline", UniformType::Float2),
            ],
        },
    }
}

#[repr(C)]
pub struct UniformsMain {
    pub mvp: glam::Mat4,
    pub playerpos: (f32, f32, f32),
    pub lightdist: f32,
}

#[repr(C)]
pub struct UniformsText {
    pub fontcolor: (f32, f32, f32, f32),
    pub actcolor: (f32, f32, f32, f32),
    pub activeline: (f32, f32),
}