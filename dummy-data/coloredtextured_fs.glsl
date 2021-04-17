#version 330 core
out vec4 FragColor;
in vec2 oTexCoords;
in vec4 oColor;
// varying ?

uniform sampler2D texture0;

void main()
{
	FragColor = oColor * texture( texture0, oTexCoords );
} 
