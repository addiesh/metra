#version 300 es

precision highp float;

in vec2 a_position;
in vec2 a_texcoord;

uniform MetraUniversal {
	float time;
	mat2 transform;
};

out highp vec2 v_texcoord;

void main() {
	gl_Position = vec4((transform * a_position), 0.0, 1.0);
	v_texcoord = a_texcoord;
}