#version 330 core
out vec4 FragColor;
in vec4 oColor;
// varying ?

void main()
{
//	FragColor = vec4( 1.0 );
	FragColor = oColor;
} 
