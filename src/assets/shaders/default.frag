#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 fragInColor;
layout(location = 0) out vec4 fragOutColor;

void main() {
    fragOutColor = vec4(fragInColor, 1.0);
}