#version 300 es

precision highp float;

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_texcoord;

uniform float metra_time;
uniform mat2 metra_transform;

//layout(binding = 0, std140) uniform MetraUniversal {
//	// offset = 0, size = 4
//	float time;
//	// offset = 4, size = 16
//	mat2 transform;
//	// offset = 20
//};

out highp vec2 v_texcoord;

void main() {
//	gl_Position = vec4((metra_transform * a_position), 0.0, 1.0);
	gl_Position = vec4(a_position, 0.0, 1.0);
	v_texcoord = a_texcoord;
}