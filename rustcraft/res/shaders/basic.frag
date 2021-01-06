#version 330 core

layout (location = 0) out vec4 color;

in vec4 v_Position;
in vec2 v_TexCoord;
in vec3 v_Normal;
in vec2 v_TileCoord;

uniform sampler2D u_Texture;

void main()
{
    vec2 tileSize = vec2(1, 1) / 16;
    vec2 tileOffset = v_TileCoord;
//    vec2 tileOffset = vec2(0, 15);

    vec2 textureCoord = (tileOffset * tileSize) + mod(v_TexCoord, 1.0) * tileSize;

    vec4 texColor = texture(u_Texture, textureCoord);
    color = texColor;
}