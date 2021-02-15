#version 330 core

layout (location = 0) out vec4 color;

in vec4 v_Position;
in vec2 v_TexCoord;
in vec3 v_Normal;
in vec2 v_TileCoord;

uniform sampler2DArray u_Texture;

void main() {
    vec2 tileUV = vec2(dot(vec3(v_Normal.y-v_Normal.z, 0, v_Normal.x), vec3(v_Position)),
                       dot(vec3(0, abs(v_Normal.x+v_Normal.z), v_Normal.y), vec3(v_Position)));

    float tileSize = (1.0/16.0);
    vec2 tileOffset = v_TileCoord / 16.0;
    float layer = v_TileCoord.y * 16.0 + v_TileCoord.x;

    vec4 texColor = texture(u_Texture, vec3(tileOffset + fract(tileUV), layer));
    color = texColor;
}