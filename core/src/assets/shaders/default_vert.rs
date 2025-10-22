// @todo - replace this with loading glsl files instead
pub const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 300 es
precision highp float;

layout(location = 0) in vec3 aPosition;

uniform mat4 uProjectionMatrix;
uniform mat4 uViewMatrix;
uniform mat4 uModelMatrix;

void main() {
  gl_Position = uProjectionMatrix * uViewMatrix * uModelMatrix * vec4(aPosition, 1.0);
}
\0";
