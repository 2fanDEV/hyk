#version 450

#extension GL_GOOGLE_include_directive : require
#include "scene_data_input.glsl"

layout (location = 0) in vec3 inPosition;
layout (location = 1) in vec3 inNormal;
layout (location = 2) in vec2 inUV;
layout (location = 3) in vec4 inColor;


layout (location = 0) out vec3 outNormal;
layout (location = 1) out vec3 outColor;
layout (location = 2) out vec2 outUV;

//push constants block
layout( push_constant ) uniform constants
{
	mat4 render_matrix;
} PushConstants;

void main() 
{
	vec4 position = vec4(inPosition, 1.0f);

	gl_Position =  sceneData.viewproj * PushConstants.render_matrix *position;

	outNormal = (PushConstants.render_matrix * vec4(inNormal, 0.f)).xyz;
	outColor = inColor.xyz * materialData.colorFactors.xyz;	
	outUV = inUV;
}

