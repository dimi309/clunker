#version 420
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) out vec4 outputColour;

void main() {
  outputColour = vec4(0.1, 0.1, 0.7, 1.0);
}
