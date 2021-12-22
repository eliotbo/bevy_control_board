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

layout(set = 2, binding = 6) uniform KnobShader_size{
    vec2 qsize;
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



float Function(float x) {
    float freq = PI*2;
    return abs(0.9*sin(freq*x));
}

float func2d(vec2 x)                                 
{
    float y3 = x.y  -  Function(x.x) ;
    return y3;
}  


void main( )
{
    vec2 pos = vec2(0.5, 0.5);
    vec2 uv = (Vertex_Uv.xy-pos);
    uv.y = -uv.y ;
    uv.x = uv.x * qsize.x / qsize.y;
    uv = uv * 1.4;
    uv.x += 0.6;
    uv.y += 0.4;





    vec4 red2 = vec4(1.0, 0.0, 0.0, 1.0);
    vec4 yellow = vec4(0.89, 0.41, 0.14, 1.0);
    vec4 green = vec4(0.0, 1.0, 0.0, 1.0);
    vec4 bluish = vec4(0.13, 0.28, 0.86, 1.0);

    vec4 red = vec4(0.0, 0.0, 0.0, 0.00);
    vec4 color2 = color;
    // color2.a = 0.3;
    vec4 black = vec4(0.0, 0.0, 0.0, 1.0);

    float bg_mul = 0.5;
    vec4 colBackground1 = vec4( 0.92, 0.96, 0.9, 1.0)* 0.85;
    vec4 colBackground2 = vec4( 0.87, 0.93, 0.83, 1.0)* 0.9;
    
    float tile_freq = 4.0;
    vec4 rect = mix(colBackground1, colBackground2, mod(floor(tile_freq*uv.x)+floor(tile_freq*uv.y), 2.0));

    ///////////////////// axes /////////////////////
    float axes_thickness = 0.005;
    float smooth_dist = 0.002;
    float ax_offset = 0.1;


    float ax_h = sdSegment(uv, vec2(0, -ax_offset ), vec2(1.0, -ax_offset ) ); 
    float hor_line = smoothstep(-smooth_dist+axes_thickness, smooth_dist+axes_thickness, ax_h);
    rect = mix(black*0.3, rect, hor_line);


    float ax_v = sdSegment(uv, vec2(-ax_offset, 0 ), vec2( -ax_offset, 1.0 ) ); 
    float ver_line = smoothstep(-smooth_dist+axes_thickness, smooth_dist+axes_thickness, ax_v);

    rect = mix(black*0.3, rect,    ver_line );
    ///////////////////// axes /////////////////////





    /////////////////// simple derivative correction /////////////////
    float epsilon = 0.001;
    float dy = (Function(uv.x+epsilon*0.5)-Function(uv.x-epsilon*0.5))/epsilon;
    float left_dy = (Function(uv.x)-Function(uv.x-epsilon))/epsilon;
    float right_dy = (Function(uv.x+epsilon)-Function(uv.x))/epsilon;

    if (sign(left_dy) != sign(right_dy)) {
        dy = max(abs(left_dy), abs(right_dy));
    }

    float solid = 0.0131;
    float smooth_dist2 = 0.006 ;
    float fun = uv.y - Function(uv.x);
    float grad_corrected = abs(fun)/sqrt(1.0+dy*dy);

    // if ((sign(left_dy) != sign(right_dy)) && (abs(left_dy) > 1.0 || abs(right_dy) > 1.0 ) && (fun < 0.0)) {
    //     grad_corrected  = 2.0;
    // }

    float correction = cos(atan(dy));
    float a = abs(fun * correction);
    float plotDerivative = smoothstep(solid, solid + smooth_dist2, a);
    // float plotDerivative = smoothstep(solid, solid + smooth_dist2, grad_corrected);
    float gatex =  smoothstep(-0.005, 0.005, uv.x) * (1 - smoothstep(-0.005, 0.005, uv.x - 1.0));

    // for a small enough curve width, we could assume dy(x) ~= dy(x + width)
    // In that case, the normal to the derivative gives the direction of smallest distance between x and the curve

    vec2 h = vec2( epsilon, 0.0 );
    float dx2 = (func2d(uv+h.xy) - func2d(uv-h.xy))/(2.0*h.x);
    float dy2 = (func2d(uv+h.yx) - func2d(uv-h.yx))/(2.0*h.x);
    // vec2 grad = vec2( dx2 , dy2  );
    vec2 normal = normalize(vec2(-dy2, dx2));

    // case of the point is above the cuve
    // x 

    // if (dy2 > 0) {
    //      if (fun < 0.0)  {
    //         normal = -normal;
    //     } 
    // } else {
    //     // if (fun > 0.0)  {
    //         // normal = -normal;
    //     // } 
    // }
   

    // if ((fun > 0.0) && (dy2 > 0)) {
    //     normal = -normal;
    // }
    
    float dir = 0.7;
    if (fun < 0.0) {
        dir = -dir;
    }


    float theta = atan(dy);
    float deltax = a * sin(theta);


    float x1 = uv.x - deltax ;
    if (fun > 0.0) {
        x1 = uv.x + deltax ;
    }
    // float x1 = uv.x - dir * abs(normal.x) * a ;

    float ss = 0.005 * correction;
    float zz = zoom;
    // zz  = 0.2;
    float gatex2 =  smoothstep(-ss, ss, x1- zz) * (1 - smoothstep(-ss, ss, x1 - (1-zz)));


    rect = 	mix(rect, bluish, gatex2 * (1-plotDerivative));

    /////////////////// simple derivative correction /////////////////

   


    // // vignetting	
	rect *= 1.0 - 0.1*length(uv);

    o_Target = rect;


    
    
}

// void main( )
// {
//     vec2 pos = vec2(0.5, 0.5);
//     vec2 uv_original = (Vertex_Uv.xy-pos);



//     vec2 p = uv_original * 2; 
    


//     float smooth_dist = 0.02;


//     float segh = sdSegment(p, vec2(0.0,0.0), vec2(0.0,0.99));
//     float segv = sdSegment(p, vec2(0.0,0.0), vec2(0.99,0.0));


//     vec4 background_color = vec4(clear_color.xyz, 0.0);
//     vec4 color_maybe_disabled = color;
//     vec4 black = vec4(0.0,0.0,0.0,1.0);

//     // enabled vs disabled color
//     if (zoom == 0.0) {
//         color_maybe_disabled = mix(color, background_color, 0.25); // vec4(color.xyz*0.5, 1.0);
//         black = mix(black, background_color, 0.25); 
//     }

    
// 	// vec4 col = mix( background_color, color_maybe_disabled, 1.0-smoothstep(0.0,smooth_dist,d) );
//     vec4 col = mix(background_color , black, 1.0 - smoothstep(0.0,smooth_dist,segh));
//     col = mix(col , black, 1.0 - smoothstep(0.0,smooth_dist,segv));

//     // vec4 col = mix(col , black, 1.0 - smoothstep(0.0,smooth_dist,seg));



// 	o_Target = col;

//     // o_Target = vec4(1.0, 1.0, 0.0, 1.0);


// }
