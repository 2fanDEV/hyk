#version 450

layout(location = 1) in vec2 v_texCoords;
layout(set = 0, binding = 0) uniform texture2D image_texture;
layout(set = 0, binding = 0) uniform sampler image_sampler; // Use a name indicating image

layout(location = 0) out vec4 f_outColor;

void main() {
    vec4 sampled_data = texture(sampler2D(image_texture, image_sampler), v_texCoords);
    f_outColor = sampled_data;
}
