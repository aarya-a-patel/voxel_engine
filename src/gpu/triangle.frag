#version 330 core
#define M_PI 3.1415926535897932384626433832795

//out vec4 Color;

uniform sampler3D ourTexture;

ivec3 fixCoord(ivec3);

void main()
{
    vec2 uv = gl_FragCoord.xy / vec2(800, 600);
    vec2 start_angle = vec2(M_PI / 3.0, M_PI / 4.0); // Replace with camera angle
    vec2 a = vec2(uv * M_PI / 2.0 + start_angle);
    vec3 pos = vec3(10.0, 0.0, 0.0);
    pos.x = -pos.x;
    vec3 ray = vec3(sin(a.x) * cos(a.y), cos(a.x), sin(a.x) * sin(a.y));
    vec3 delta_dist = abs(vec3(length(ray)) / ray);
    ivec3 step = ivec3(sign(ray));
    ivec3 map = ivec3(floor(pos)); // Replace vec3(0.0) with position
    vec3 side_dist = (sign(ray) * (vec3(map) - pos) + (sign(ray) * 0.5) + 0.5) * delta_dist;

    bvec3 mask;

    while (length(map) < 64 && texelFetch(ourTexture, fixCoord(map), 0).rgb == vec3(0.0)) {
        mask = lessThanEqual(side_dist.xyz, min(side_dist.yzx, side_dist.zxy));
        side_dist += vec3(mask) * delta_dist;
        map += ivec3(vec3(mask)) * step;
    }

    gl_FragColor = texelFetch(ourTexture, fixCoord(map), 0);
}

ivec3 fixCoord(ivec3 pos) {
    pos.x = -pos.x;
    return pos.yxz + ivec3(64);
}