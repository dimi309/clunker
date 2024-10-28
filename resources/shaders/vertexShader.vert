#version 420
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec4 position;
layout(location = 1) in vec3 normal;

layout(binding = 0) uniform ubo {
  mat4 transformation;
  vec3 offset;
};

layout(location = 0) smooth out float cosAngIncidence;

void main()
{
  vec4 lightDir = vec4(0.3, 0.4, 0.2, 1.0);

  cosAngIncidence = dot(vec4(normal, 1), lightDir);
  
  gl_Position = position + vec4(offset, 0.0);

}
