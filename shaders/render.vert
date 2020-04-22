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



layout(location = 0) out vec3 f_pos;
layout(location = 1) out vec3 f_normal;
layout(location = 2) out vec2 f_texture_coords;
layout(location = 3) out mat3 f_tbn;


void main() {
    BoidData boid = boid_datas.boids[gl_InstanceIndex];

    vec4 pos = vec4(v_pos, 1.0);

    gl_Position = u_projection * u_view *  boid.model * pos;
    f_pos = pos.xyz;
    f_normal = v_normal;
    f_texture_coords = v_texture_cords;


    int face_index = gl_VertexIndex / 3;
    vec3 face_tangent = b_face_datas[face_index].tangent.xyz;

    vec3 T = normalize(vec3(boid.model * vec4(face_tangent, 0.0)));
    vec3 N = normalize(vec3(boid.model * vec4(v_normal, 0.0)));
    T = normalize(T - dot(T, N) * N);

    vec3 B = cross(N, T);

    f_tbn = mat3(T, B, N);
}