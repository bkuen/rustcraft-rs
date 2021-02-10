#version 330 core

layout (location = 0) out vec4 color;

in vec4 v_Position;
in vec2 v_TexCoord;
in vec3 v_Normal;
in vec2 v_TileCoord;

uniform sampler2D u_Texture;

void main() {

//    vec2 tileUV = vec2(dot(v_Normal.zxy, vec3(v_Position)),
//                       dot(v_Normal.yzx, vec3(v_Position)));

    vec2 tileUV = vec2(dot(vec3(v_Normal.y-v_Normal.z, 0, v_Normal.x), vec3(v_Position)),
                       dot(vec3(0, abs(v_Normal.x+v_Normal.z), v_Normal.y), vec3(v_Position)));

    float tileSize = (1.0/16.0);
    vec2 tileOffset = v_TileCoord / 16.0;
    vec2 texCoord = tileOffset + tileSize * fract(tileUV);

    vec4 texColor = texture(u_Texture, texCoord);
    color = texColor;
}

//void main()
//{
//    vec2 tileSize = vec2(1, 1) / 16;
//    vec2 tileOffset = v_TileCoord;
////    vec2 tileOffset = vec2(0, 15);
//
////    float textureX = (tileOffset.x * tileSize) + mod(v_TexCoord.x, 1.0) * tileSize;
////    float textureY = (tileOffset.y * tileSize) + mod(v_TexCoord.y, 1.0) * tileSize;
//
//    vec2 textureCoord = (tileOffset * tileSize) + mod(v_TexCoord, 1.0) * tileSize;
////    vec2 textureCoord = vec2(textureX, textureY);
//
//    vec4 texColor = texture(u_Texture, textureCoord);
//    color = texColor;
//}