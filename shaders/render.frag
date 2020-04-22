#version 450 core
layout(location = 0) in vec3 f_pos;
layout(location = 1) in vec3 f_normal;
layout(location = 2) in vec2 f_texture_coords;
layout(location = 3) in mat3 f_tbn;

layout(set = 0, binding = 0) uniform MaterialData {
    vec4 u_ambient;
    vec4 u_diffuse;
    vec4 u_specular;
    float u_alpha;
    float u_shininess;
};

layout(set = 0, binding = 1) uniform texture2D t_ambient;
layout(set = 0, binding = 2) uniform sampler s_ambient;

layout(set = 0, binding = 3) uniform texture2D t_diffuse;
layout(set = 0, binding = 4) uniform sampler s_diffuse;

layout(set = 0, binding = 5) uniform texture2D t_specular;
layout(set = 0, binding = 6) uniform sampler s_specular;

layout(set = 0, binding = 7) uniform texture2D t_normal;
layout(set = 0, binding = 8) uniform sampler s_normal;

layout(set = 0, binding = 9) uniform texture2D t_alpha;
layout(set = 0, binding = 10) uniform sampler s_alpha;

layout(set = 2, binding = 0) uniform ProjectionViewData {
    mat4 u_projection;
    mat4 u_view;
    vec3 u_camera;
};

layout(location = 0) out vec4 o_color;

vec4 directional_light(vec3 light_dir, vec3 normal, vec3 camera_dir) {
    vec4 diffuse_texture = texture(sampler2D(t_diffuse, s_diffuse), f_texture_coords);

    float alpha = u_alpha * texture(sampler2D(t_alpha, s_alpha), f_texture_coords).x * diffuse_texture.w;

    if (alpha < 0.1) {
        discard;
    }

    vec3 ligth_reflect_dir = reflect(-light_dir, normal);

    float diff = clamp(dot(normal, light_dir), 0.0, 1.0);
    float spec = pow(clamp(dot(camera_dir, ligth_reflect_dir), 0.0, 1.0), u_shininess);


    vec3 ambient = texture(sampler2D(t_ambient, s_ambient), f_texture_coords).xyz * u_ambient.xyz;
    vec3 diffuse = diffuse_texture.xyz * u_diffuse.xyz * diff * (1.0 - spec);
    vec3 specular = texture(sampler2D(t_specular, s_specular), f_texture_coords).xyz * u_specular.xyz * spec;

    return vec4(ambient + diffuse + specular, alpha);
}

void main() {
    vec3 normal = normalize(f_normal);
    vec4 normal_texture = texture(sampler2D(t_normal, s_normal), f_texture_coords);

    if (normal_texture.w != 0.0) {
        vec3 normal = normalize(f_tbn * (normal_texture.xyz * 2.0 - 1.0));
    }

    vec3 camera_dir = normalize(u_camera - f_pos);

    vec4 color = directional_light(normalize(vec3(0.0, 1.0, 1.0)), normal, camera_dir);

    float gamma = 1.0 / 0.7;
    color.xyz = pow(color.xyz, vec3(gamma));

    o_color = color;
}