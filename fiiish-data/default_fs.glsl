#version 330 core
out vec4 FragColor;
in vec2 screen_pos;

void main()
{
    FragColor = vec4(1.0f, abs(screen_pos.x*2.0), abs(screen_pos.y*2.0), 0.125f);
} 
