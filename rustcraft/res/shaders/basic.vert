#version 330 core

layout (location = 0) in vec4 position;
layout (location = 1) in vec2 texCoord;
layout (location = 2) in vec3 normal;
layout (location = 3) in vec2 tileCoord;

out vec4 v_Position;
out vec2 v_TexCoord;
out vec3 v_Normal;
out vec2 v_TileCoord;

uniform mat4 u_MVP;

void main()
{
    v_Position = position;
    gl_Position = u_MVP * position;
    v_TexCoord = texCoord;
    v_Normal = normal;
    v_TileCoord = tileCoord;
}
