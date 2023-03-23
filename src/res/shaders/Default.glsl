#shader vertex
#version 330 core

layout(location = 0) in vec4 position;
layout(location = 1) in vec2 texture_Coord;

out vec2 v_TextureCoord;

uniform mat4 u_MVP;

void main() {
    gl_Position = u_MVP * position;
    v_TextureCoord = texture_Coord;
}

#shader fragment
#version 330 core

in vec2 v_TextureCoord;

uniform vec4 u_Color;
uniform sampler2D u_Texture;

out vec4 Color;

void main() {
   Color = texture(u_Texture, v_TextureCoord);
}
