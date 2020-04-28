#version 450 core
layout(location = 0) in vec3 v_pos;
layout(location = 1) in vec3 v_normal;
layout(location = 2) in vec2 v_texture_cords;

struct FaceData {
    vec4 tangent;
};

layout(set = 1, binding = 0) buffer FaceDatas {
    FaceData b_face_datas[];
};

layout(set = 2, binding = 0) uniform ProjectionViewData {
    mat4 u_projection;
    mat4 u_view;
    vec3 u_camera;
};

struct BoidData {
    vec4 pos;
    vec4 vel;
    mat4 model;
};

layout(set = 2, binding = 1) buffer BoidDatas {
    BoidData boids[];
} boid_datas;

layout(set = 2, binding = 2) uniform TimeData {
    float time;
};

layout(location = 0) out vec3 f_pos;
layout(location = 1) out vec3 f_normal;
layout(location = 2) out vec2 f_texture_coords;
layout(location = 3) out mat3 f_tbn;


const float HEAD_LIMIT = -0.35;
const vec3 SPEED = vec3(5.0, 4.5, 1.0);
const vec3 FREQUENCY = vec3(1.4, 1.0, 0.2);
const vec3 AMPLITUDE = vec3(0.2, 0.05, 0.05);


void wiggle(inout vec4 pos, vec3 vel) {
    float speed = clamp(length(vel), 0.1, 1.2);

    pos.z += (sin((pos.z + time * SPEED.z) * FREQUENCY.z) * AMPLITUDE.z) * speed;
    pos.y += (sin((pos.z + time * SPEED.y) * FREQUENCY.y) * AMPLITUDE.y) * speed;

    if (pos.z < HEAD_LIMIT) {
        pos.x += (sin((0.05 + time * SPEED.x) * FREQUENCY.x) * AMPLITUDE.x * HEAD_LIMIT) * speed;
    } else {
        pos.x += (sin((pos.z + time * SPEED.x) * FREQUENCY.x) * AMPLITUDE.x * pos.z) * speed;
    }
}

mat3 calculate_tangent(mat4 model) {
    int face_index = gl_VertexIndex / 3;
    vec3 face_tangent = b_face_datas[face_index].tangent.xyz;

    vec3 T = normalize(vec3(model * vec4(face_tangent, 0.0)));
    vec3 N = normalize(vec3(model * vec4(v_normal, 0.0)));
    T = normalize(T - dot(T, N) * N);

    vec3 B = cross(N, T);

    return mat3(T, B, N);
}

void main() {
    BoidData boid = boid_datas.boids[gl_InstanceIndex];

    vec4 pos = vec4(v_pos, 1.0);
    wiggle(pos, boid.vel.xyz);


    gl_Position = u_projection * u_view *  boid.model * pos;
    f_pos = pos.xyz;
    f_normal = v_normal;
    f_texture_coords = v_texture_cords;


    f_tbn = calculate_tangent(boid.model);
}