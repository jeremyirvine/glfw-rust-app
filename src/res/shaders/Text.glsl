#shader vertex
#version 330 core

layout(location = 0) in vec4 position;
layout(location = 1) in vec2 texture_Coord;
layout(location = 2) in float texture_Index;

out vec2 v_TextureCoord;
out float v_TextureIndex;

uniform mat4 u_MVP;

void main() {
    gl_Position = u_MVP * position;
    v_TextureCoord = texture_Coord;
    v_TextureIndex = texture_Index;
}

#shader fragment
#version 330 core

in vec2 v_TextureCoord;
in float v_TextureIndex;

uniform sampler2D u_Textures[2];

out vec4 Color;

void main() {
    int index = int(v_TextureIndex);
    Color = texture(u_Textures[index], v_TextureCoord);    
}
