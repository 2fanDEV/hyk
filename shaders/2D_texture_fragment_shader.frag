#version 450

layout(set = 0, binding = 0) uniform texture2D image_texture;
layout(set = 0, binding = 1) uniform sampler image_sampler; 

layout(location = 0) out vec4 f_outColor;
layout(location = 1) in vec2 v_texCoords;

void main() {
    vec4 sampled_data = texture(sampler2D(image_texture, image_sampler), v_texCoords);
    f_outColor = sampled_data;
}
