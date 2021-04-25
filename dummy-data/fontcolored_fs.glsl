#version 330 core
out vec4 FragColor;
in vec2 oTexCoords;
in vec4 oColor;
// varying ?

uniform sampler2D texture0;

const float innerEdgeCenter = 0.5;

//const float smoothing = ( 1.0/4.0 ) + 0.3;

void main()
{
	float dx = length(dFdx(oTexCoords.xy));
	float dy = length(dFdy(oTexCoords.xy));
	float smoothing = 16.0*0.5*(dx+dy) + 0.3;

	float distance = texture( texture0, oTexCoords ).a;
	float alpha = smoothstep( innerEdgeCenter - smoothing, innerEdgeCenter + smoothing, distance ) * oColor.a;

	FragColor = vec4( oColor.rgb, alpha );
} 
