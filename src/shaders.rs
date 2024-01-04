use miniquad::*;
use glam;

pub const VERTEX_MAIN: &str = r#"#version 330 core
in vec3 pos;
in vec2 uv;
in float act;

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

pub const VERTEX_OVERLAY: &str = r#"#version 330 core
in vec3 pos;
in vec2 uv;
in float act;

out vec2 texcoord;

void main() {
    gl_Position = vec4((pos.x-0.5)*2.0, (0.5-pos.y)*2.0, 0.0, 1.0);
    texcoord = uv;
}"#;

pub const FRAGMENT_OVERLAY: &str = r#"#version 330 core
in vec2 texcoord;

out vec4 FragColor;

uniform sampler2D tex;
uniform vec4 fontcolor;

vec4 col;

void main() {
    col = texture(tex, texcoord);

    if (col.x+col.y+col.z > 2.99) {
        discard;
    } else {
        FragColor = fontcolor;
        
    }
}"#;

pub const VERTEX_GUI: &str = r#"#version 330 core
in vec3 pos;
in vec2 uv;
in float act;

uniform vec4 fontcolor;
uniform vec4 actcolor;

out vec2 texcoord;
out vec4 cols;

void main() {
    gl_Position = vec4((pos.x-0.5)*2.0, (0.5-pos.y)*2.0, 0.0, 1.0);
    texcoord = uv;
    if (act > 0.0) {
        cols = actcolor;
    } else {
        cols = fontcolor;
    }
}"#;

pub const FRAGMENT_GUI: &str = r#"#version 330 core
in vec2 texcoord;
in vec4 cols;

out vec4 FragColor;

uniform sampler2D tex;

vec4 col;

void main() {
    col = texture(tex, texcoord);

    if (col.x+col.y+col.z > 2.99) {
        discard;
    } else {
        if (col.x+col.y+col.z < 0.01) {
            FragColor = cols;
        } else {
            FragColor = col;
        }
    }
}"#;

pub const VERTEX_MAP: &str = r#"#version 330 core
in vec3 pos;
in vec2 uv;
in float act;

uniform vec4 fontcolor;
uniform vec4 actcolor;

out vec2 texcoord;
out vec4 cols;
out vec2 spos;

void main() {
    gl_Position = vec4((pos.x-0.5)*2.0, (0.5-pos.y)*2.0, 0.0, 1.0);
    spos = pos.xy;
    texcoord = uv;
    cols = fontcolor;
    if (act > 0.9 && act <= 1.1) {
        cols = actcolor;
    }
    if (act > 1.9 && act <= 2.1) {
        cols = actcolor;
    }
    if (act > 2.9 && act <= 3.1) {
        cols = vec4(0.8, 0.0, 0.2, 1.0);
    }
}"#;

pub const FRAGMENT_MAP: &str = r#"#version 330 core
in vec2 texcoord;
in vec4 cols;
in vec2 spos;

out vec4 FragColor;

uniform sampler2D tex;
uniform vec4 cent;

vec4 col;

void main() {
    col = texture(tex, texcoord);

    if (length((spos.xy-cent.xy)/cent.zw) > 1.0) {
        discard;
    } else {
        if (col.x+col.y+col.z > 2.99) {
            FragColor = col;
        } else {
            FragColor = cols;
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

pub fn meta_overlay() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("fontcolor", UniformType::Float4),
            ],
        },
    }
}

pub fn meta_gui() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("fontcolor", UniformType::Float4),
                UniformDesc::new("actcolor", UniformType::Float4),
            ],
        },
    }
}

pub fn meta_map() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("fontcolor", UniformType::Float4),
                UniformDesc::new("actcolor", UniformType::Float4),
                UniformDesc::new("cent", UniformType::Float4),
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
pub struct UniformsOverlay {
    pub fontcolor: (f32, f32, f32, f32),
}

#[repr(C)]
pub struct UniformsGUI {
    pub fontcolor: (f32, f32, f32, f32),
    pub actcolor: (f32, f32, f32, f32),
}

#[repr(C)]
pub struct UniformsMap {
    pub fontcolor: (f32, f32, f32, f32),
    pub actcolor: (f32, f32, f32, f32),
    pub cent: (f32, f32, f32, f32)
}
