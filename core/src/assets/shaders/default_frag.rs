// @Todo
// replace this with loading glsl files instead
pub const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 300 es
precision highp float;

out vec4 fragColor;

void main() {
    fragColor = vec4(0.3, 0.5, 0.7, 1.0);
}
\0";
