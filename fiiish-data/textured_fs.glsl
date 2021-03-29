#version 330 core
out vec4 FragColor;
in vec2 oTexCoords;

void main()
{
	// :TODO: texture lookup

    FragColor = vec4(0.0f, oTexCoords.x, oTexCoords.y, 1.0f);
} 
