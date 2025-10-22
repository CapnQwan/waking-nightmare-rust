// @todo - replace this with loading glsl files instead
pub const LIT_FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 300 es
precision mediump float;

// Declare an output variable for the fragment color
in vec3 vNormal;
in vec3 vPosition;

uniform vec3 uViewPosition;

out vec4 fragColor;

void main() {
  vec3 uLightPosition = vec3(-2.0, 2.0, -5.0);
  vec3 uLightAmbient = vec3(0.5);
  vec3 uLightDiffuse = vec3(0.2);
  vec3 uLightSpecular = vec3(0.75);
  vec3 uMaterialAmbient = vec3(0.2);
  vec3 uMaterialDiffuse = vec3(0.8);
  vec3 uMaterialSpecular = vec3(1.0);
  float uMaterialShininess = 16.0;

  vec3 normal = normalize(vNormal);
  vec3 lightDir = normalize(uLightPosition - vPosition);
  vec3 viewDir = normalize(uViewPosition - vPosition);
  vec3 reflectDir = reflect(-lightDir, normal);

  // Ambient
  vec3 ambient = uLightAmbient * uMaterialAmbient;

  // Diffuse
  float diff = max(dot(normal, lightDir), 0.0);
  vec3 diffuse = uLightDiffuse * uMaterialDiffuse * diff;

  // Specular
  float spec = pow(max(dot(viewDir, reflectDir), 0.0), uMaterialShininess);
  vec3 specular = uLightSpecular * uMaterialSpecular * spec;

  // Combine
  vec3 color = ambient + diffuse + specular;
  fragColor = vec4(color, 1.0);
  //fragColor = vec4(normalize(vNormal) * 0.5 + 0.5, 1.0); // normals to color
  //fragColor = vec4(abs(vPosition), 1.0); // positions to color
}
\0";
