#version 330 core

layout (location = 0) out vec4 color;

in vec4 v_Position;
in vec2 v_TexCoord;
in vec3 v_Normal;
in vec2 v_TileCoord;

// uniform sampler2D u_Texture;
uniform sampler2DArray u_Texture;

//vec4 fourTapSample(vec2 tileOffset, vec2 tileUV, float tileSize, sampler2D atlas);

//void main222() {
//
//  vec2 uv      = vec2(dot(vec3(v_Normal.y-v_Normal.z, 0, v_Normal.x), vec3(v_Position)),
//                      dot(vec3(0, abs(v_Normal.x+v_Normal.z), v_Normal.y), vec3(v_Position)));
//
//  vec4 color   = vec4(0,0,0,0);
//  float weight = 0.0;
//
//  float ambientOcclusion = 0.8;
//  float tileSize = 16.0;
//  float tileCount = 16.0;
//
//  vec2 tileOffset = 2.0 *  tileSize * v_TileCoord;
//  float denom     = 2.0 * tileSize * tileCount;
//
//  for(int dx=0; dx<2; ++dx) {
//    for(int dy=0; dy<2; ++dy) {
//      vec2 offset = 2.0 * fract(0.5 * (uv + vec2(dx, dy)));
//      float w = pow(1.0 - max(abs(offset.x-1.0), abs(offset.y-1.0)), 16.0);
//
//      vec2 tc = (tileOffset + tileSize * offset) / denom;
//      color  += w * texture2D(u_Texture, tc);
//      weight += w;
//    }
//  }
//  color /= weight;
//
//  if(color.w < 0.5) {
//    discard;
//  }
//
//  float light = ambientOcclusion + max(0.15*dot(v_Normal, vec3(1,1,1)), 0.0);
//
//  gl_FragColor = vec4(color.xyz * light, 1.0);
//}
//
//void main333() {
//    vec2 tileUV = vec2(dot(vec3(v_Normal.y-v_Normal.z, 0, v_Normal.x), vec3(v_Position)),
//                       dot(vec3(0, abs(v_Normal.x+v_Normal.z), v_Normal.y), vec3(v_Position)));
//
//    float tileSize = (1.0/16.0);
//    vec2 tileOffset = v_TileCoord / 16.0;
//    vec2 texCoord = tileOffset + tileSize * fract(tileUV);
//
//    vec4 texColor = texture(u_Texture, texCoord);
//    // vec4 texColor = fourTapSample(tileOffset, tileUV, tileSize, u_Texture);
//    color = texColor;
//}

void main() {
//    vec2 tileUV = vec2(dot(v_Normal.zxy, vec3(v_Position)),
//                       dot(v_Normal.yzx, vec3(v_Position)));

    vec2 tileUV = vec2(dot(vec3(v_Normal.y-v_Normal.z, 0, v_Normal.x), vec3(v_Position)),
                       dot(vec3(0, abs(v_Normal.x+v_Normal.z), v_Normal.y), vec3(v_Position)));

    float tileSize = (1.0/16.0);
    vec2 tileOffset = v_TileCoord / 16.0;
    vec2 texCoord = tileOffset + tileSize * fract(tileUV);

    vec2 coord = (v_TileCoord + v_TexCoord * 16)/16.0;

    vec4 texColor = texture(u_Texture, vec3(tileOffset + tileSize * v_TexCoord + fract(tileUV), 0));
//    vec4 texColor = texture(u_Texture, vec3(texCoord, 0));
//    vec4 texColor = texture(u_Texture, texCoord);
    color = texColor;
}

//// Ported from: https://0fps.net/2013/07/09/texture-atlases-wrapping-and-mip-mapping/
//vec4 fourTapSample(vec2 tileOffset,
//                   vec2 tileUV,
//                   float tileSize,
//                   sampler2D atlas)
//{
//    // Initialize accumulators
//    vec4 color = vec4(0.0, 0.0, 0.0, 0.0);
//    float totalWeight = 0.0;
//
//    for(int dx=0; dx<2; ++dx) {
//        for(int dy=0; dy<2; ++dy) {
//            // Compute coordinate in 2x2 tile patch
//            vec2 tileCoord = 2.0 * fract(0.5 * (tileUV + vec2(dx, dy)));
//
//            // Weight sample based on distance to center
//            float w = pow(1.0 - max(abs(tileCoord.x-1.0), abs(tileCoord.y-1.0)), 16.0);
//
//            // Compute atlas coord
//            vec2 atlasUV = (tileOffset + tileSize * tileCoord);
//
//            // Sample and accumulate
//            color += w * texture2D(atlas, atlasUV);
//            totalWeight += w;
//        }
//    }
//
//    // Return weighted color
//    return color / totalWeight;
//}