#version 300 es

precision highp float;

in highp vec2 v_texcoord;
out vec4 out_color;

void main() {
	out_color = vec4(v_texcoord, 0.0, 1.0);
}