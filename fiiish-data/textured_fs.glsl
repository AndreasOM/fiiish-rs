#version 330 core
out vec4 FragColor;
in vec2 oTexCoords;
// varying ?

uniform sampler2D texture0;

void main()
{
	// :TODO: texture lookup

	FragColor = texture( texture0, oTexCoords );
//	FragColor = vec4( 0.0, oTexCoords.x, oTexCoords.y, 1.0 );
} 
