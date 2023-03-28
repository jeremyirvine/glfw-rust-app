#shader vertex
#version 450 core

layout(location = 0) in vec4 position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec2 textureCoord;
layout(location = 3) in float textureIndex;

out vec4 fsh_Color;
out vec2 fsh_TextureCoord;
out float fsh_TextureIndex;

uniform mat4 u_MVP;

void main() {
    gl_Position = u_MVP * position;
    fsh_Color = color;
    fsh_TextureCoord = textureCoord;
    fsh_TextureIndex = textureIndex;
}

#shader fragment
#version 450 core

in vec4 fsh_Color;
in vec2 fsh_TextureCoord;
in float fsh_TextureIndex;

out vec4 Color;

uniform sampler2D u_Texture0;
uniform sampler2D u_Texture1;

void main() {
    int index = int(fsh_TextureIndex);

    if (fsh_TextureIndex == 0.0f) {
        Color = texture(u_Texture0, fsh_TextureCoord);
    } else {
        Color = texture(u_Texture1, fsh_TextureCoord);
    }
}
