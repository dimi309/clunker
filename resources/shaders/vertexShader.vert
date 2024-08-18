#version 420
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec4 position;
layout(location = 1) in vec3 normal;

layout(location = 0) smooth out float cosAngIncidence;

void main()
{
  vec4 lightDir = vec4(0.3, -0.4, 0.5, 1.0);

  cosAngIncidence = dot(vec4(normal, 1), lightDir);
  
  gl_Position = position;

  // OpenGL -> Vulkan viewport
  gl_Position.z = (gl_Position.z + gl_Position.w) / 2.0;
  gl_Position.y = -gl_Position.y;

}
