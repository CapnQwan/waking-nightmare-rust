// @todo - replace this with loading glsl files instead
pub const LIT_VERTEX_SHADER_SOURCE: &[u8] = b"
#version 300 es
precision mediump float;

layout(location = 0) in vec3 aPosition;
layout(location = 2) in vec3 aNormal;

out vec3 vNormal;
out vec3 vPosition;

uniform mat3 uNormalMatrix;
uniform mat4 uModelMatrix;
uniform mat4 uViewMatrix;
uniform mat4 uProjectionMatrix;

void main() {
  mat4 modelViewMatrix = uViewMatrix * uModelMatrix;
  gl_Position = uProjectionMatrix * modelViewMatrix * vec4(aPosition, 1.0);
  vNormal = normalize(uNormalMatrix * aNormal);
  vPosition = (modelViewMatrix * vec4(aPosition, 1.0)).xyz;
}
\0";
