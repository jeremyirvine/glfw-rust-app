#shader vertex
#version 330 core

layout(location = 0) in vec4 position;
layout(location = 1) in vec4 color;

out vec4 fsh_Color;

uniform mat4 u_MVP;

void main() {
    gl_Position = u_MVP * position;
    fsh_Color = color;
}

#shader fragment
#version 330 core

in vec4 fsh_Color;

out vec4 Color;

void main() {
   Color = fsh_Color;
}
