use miniquad::*;
use glam;

pub const VERTEX_MAIN: &str = r#"#version 330 core
in vec3 pos;
in vec2 uv;
in float act;

uniform mat4 mvp;
uniform vec3 playerpos;
uniform vec3 lightpos;
uniform float lightdist;

out vec2 texcoord;
out vec4 cols;

float col1, d1, d2;
vec3 dir1, dir2;

void main() {
    dir1 = pos - playerpos;
    d1 = length(dir1);
    dir2 = pos - lightpos;
    d2 = length(dir2);

    gl_Position = mvp * vec4(pos, 1.0);

    col1 = 0.1/(1.0+(d1/lightdist)*(d1/lightdist))+0.9/(1.0+(d2/lightdist)*(d2/lightdist));
    cols = vec4(col1,col1,col1,1.0);
    
    texcoord = uv;
}"#;

pub const FRAGMENT_MAIN: &str = r#"#version 330 core
in vec2 texcoord;
in vec4 cols;

out vec4 FragColor;

uniform sampler2D tex;

vec4 col;

void main() {
    col = texture(tex, texcoord);
    FragColor = vec4(col.xyz * cols.xyz, col.w);
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

    if (col.w < 0.9) {
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

    if (col.w < 0.9) {
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

out vec2 texcoord;
out vec2 spos;
out float acts;

void main() {
    gl_Position = vec4((pos.x-0.5)*2.0, (0.5-pos.y)*2.0, 0.0, 1.0);
    spos = pos.xy;
    texcoord = uv;
    acts = act;
}"#;

pub const FRAGMENT_MAP: &str = r#"#version 330 core
in vec2 texcoord;
in vec2 spos;
in float acts;

out vec4 FragColor;

uniform sampler2D tex;
uniform vec4 cent;
uniform vec4 fontcolor;
uniform vec4 actcolor;

vec4 col;

void main() {
    if (length((spos.xy-cent.xy)/cent.zw) < 0.04) {
        FragColor = vec4(0.8, 0.0, 0.2, 1.0);
    } else {
        if (length((spos.xy-cent.xy)/cent.zw) > 1.0) {
            discard;
        } else {
            if (acts > 0.1) {
                FragColor = actcolor;
            } else {
                col = texture(tex, texcoord);
                if (col.x+col.y+col.z > 2.99) {
                    FragColor = col;
                } else {
                    FragColor = fontcolor;
                }
            }
        }
    }
}"#;


pub const VERTEX_SCREEN: &str = r#"#version 330 core
in vec3 pos;
in vec2 uv;
in float act;

out vec2 texcoord;

void main() {
    gl_Position = vec4(pos, 1.0);
    texcoord = uv;
}"#;

pub const FRAGMENT_SCREEN: &str = r#"#version 330 core
in vec2 texcoord;

out vec4 FragColor;

uniform sampler2D tex;

void main() {
    FragColor = texture(tex, texcoord);
}"#;


pub fn meta_main() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("mvp", UniformType::Mat4),
                UniformDesc::new("playerpos", UniformType::Float3),
                UniformDesc::new("lightpos", UniformType::Float3),
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

pub fn meta_screen() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
            ],
        },
    }
}

#[repr(C)]
pub struct UniformsMain {
    pub mvp: glam::Mat4,
    pub playerpos: (f32, f32, f32),
    pub lightpos: (f32, f32, f32),
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

#[repr(C)]
pub struct UniformsScreen {
}
