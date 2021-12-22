#version 450
#define PI 3.1415926538

layout(location = 0) in vec4 gl_FragCoord;
layout(location = 1) in vec3 v_Position;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform KnobShader_color{
    vec4 color;
};
layout(set = 2, binding = 1) uniform KnobShader_clearcolor{
    vec4 clear_color;
};
layout(set = 2, binding = 2) uniform KnobShader_zoom{
    float zoom;
};
layout(set = 2, binding = 3) uniform KnobShader_hovered{
    float hovered;
};
layout(set = 2, binding = 4) uniform KnobShader_bounds{
    vec2 bounds;
};

layout(set = 2, binding = 5) uniform KnobShader_angle{
    float angle;
};

/////////////// unused ///////////////
float sdBox( in vec2 p, in vec2 b )
{
    vec2 d = abs(p)-b;
    return length(max(d,0.0)) + min(max(d.x,d.y),0.0);
}


float sdSegment( in vec2 p, in vec2 a, in vec2 b )
{
    vec2 pa = p-a, ba = b-a;
    float h = clamp( dot(pa,ba)/dot(ba,ba), 0.0, 1.0 );
    return length( pa - ba*h );
}


float sdSquareEdge(vec2 p, float r, float w)
{
    float d = sdBox( p, vec2(r,r) );
    float s1 = smoothstep(-0.005, 0.01, d);

    float width = 0.01;
    float s2 = smoothstep(-0.005-w, 0.002-w, d);
    return 1.0 - abs(s1-s2);
}
/////////////// unused ///////////////

float sdArc( in vec2 p,  in vec2 scb, in float ra, float rb )
{
    
    p.x = abs(p.x);
    float k = (scb.y*p.x>scb.x*p.y) ? dot(p,scb) : length(p);
    return sqrt( dot(p,p) + ra*ra - 2.0*ra*k ) - rb;
}


float sdCircle( vec2 p, float r)
{
    float d = length(p) - r;
    return d;
}

float opOnion( in vec2 p, in float w, in float r )
{
  return abs(sdCircle(p, r)) - w;
}

void main( )
{
    vec2 pos = vec2(0.5, 0.5);
    vec2 uv_original = (Vertex_Uv.xy-pos);



    vec2 p = uv_original * 2; 
    

    // animation
    // float angle2 = (angle  / 10 + 3 * PI / 2);
    // float angle3 =  (angle - bounds.x) / (bounds.y * 1.1 - bounds.x) * PI * 6;
    float angle3 = 0;
    float offset = 0.1;
    if (bounds.x == bounds.y) {
        angle3 = 0 + 3 * PI / 2;
    } else {
        angle3 = PI * (angle ) * (1 - offset )  + 3 * PI / 2;
    }

    
    float time = 10.0;
    float ta =   -angle3  - offset * PI  ; 
    float tb = angle3 + PI / 2 ; 
    float rb = 0.015;   
    float smooth_dist = 0.06;
    vec2 sca = vec2(sin(ta),cos(ta));

    p = p * mat2(sca.x,sca.y,-sca.y,sca.x);
    vec2 p2 = p * mat2(sca.x,sca.y,-sca.y,sca.x);

    
    float line_off = 0.9;
    vec2 offset_ang = vec2(sin(offset * PI * line_off + PI /2),cos(offset * PI * line_off+ PI /2));
    p2 = p2 * mat2(offset_ang.x,offset_ang.y,-offset_ang.y,offset_ang.x);


    float d = sdArc(p,vec2(sin(tb),cos(tb)), 0.7, rb);
    float dc = opOnion(p, 0.05, 0.58);
    float db = sdBox(p2 - vec2(0.0,0.4), vec2(0.03,0.2));

    vec4 background_color = vec4(clear_color.xyz, 0.0);
    vec4 color_maybe_disabled = color;
    vec4 black = vec4(0.0,0.0,0.0,1.0);

    // enabled vs disabled color
    if (zoom == 0.0) {
        color_maybe_disabled = mix(color, background_color, 0.25); // vec4(color.xyz*0.5, 1.0);
        black = mix(black, background_color, 0.25); 
    }

    
	vec4 col = mix( background_color, color_maybe_disabled, 1.0-smoothstep(0.0,smooth_dist,d) );
    col = mix(col , black, 1.0 - smoothstep(0.0,smooth_dist,dc));
    col = mix(col , black, 1.0 - smoothstep(0.0,smooth_dist,db));


	o_Target = col;


}
