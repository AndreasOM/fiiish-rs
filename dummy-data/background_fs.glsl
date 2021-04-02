#version 330 core
out vec4 FragColor;
in vec2 oTexCoords;
// varying ?

uniform sampler2D texture0;
uniform sampler2D texture1;
uniform float phase;
void main()
{
	// :TODO: texture lookup

//	vec2 tc = clamp(oTexCoords, 0.0, 0.5);
//	float i = float(int( oTexCoords.x ));
	float x = texture( texture0, oTexCoords ).r;
	float y = phase;

	vec2 tc = vec2( x, y );
	FragColor = texture( texture1, tc );

//	if ( oTexCoords.y < 0.5 ) {
//		FragColor = texture( texture0, oTexCoords );// *vec4( 0.5, 0.5, 1.0, 1.0 );// + vec4( 0.5, 0.125*i, 0.0, 0.0 );
//	} else {
//		FragColor = texture( texture1, oTexCoords );// *vec4( 0.5, 1.0, 0.5, 1.0 );// + vec4( 0.5, 0.125*i, 0.0, 0.0 );
//	}
//	FragColor = vec4( 0.0, oTexCoords.x, oTexCoords.y, 1.0 );
} 
