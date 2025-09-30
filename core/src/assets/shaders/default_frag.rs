// @todo - replace this with loading glsl files instead
pub const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

void main() {
    gl_FragColor = vec4(0.0, 0.3, 0.7, 1.0);
}
\0";
