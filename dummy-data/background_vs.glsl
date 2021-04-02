#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 texCoords;
out vec2 oTexCoords;

uniform mat4 modelViewProjectionMatrix;

void main()
{
	oTexCoords = texCoords;
    gl_Position = modelViewProjectionMatrix * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}

