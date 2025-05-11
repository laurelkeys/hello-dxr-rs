// dxc shader.hlsl /T lib_6_3 /Fo shader.o

struct Payload
{
    float3 color;
    bool allow_reflection;
    bool missed;
};

RaytracingAccelerationStructure scene_tlas : register(t0);

RWTexture2D<float4> scene_output : register(u0);

#define HELLO_CUBE_BARYCENTRICS() 0

// InstanceID.
static const uint CUBE_ID = 0;
static const uint FLOOR_ID = 1;
static const uint MIRROR_ID = 2;
static const float3 UNKNOWN_ID_HIT_COLOR = float3(1, 0, 1); // magenta

static const float3 Y_UP = float3(0, 1, 0);

//
// Hard-coded constants.
//

static const float3 camera_world = float3(0.0, 1.5, -7.0);
static const float3 light_world = float3(0.0, 200.0, 0.0);
static const float3 sky_top = float3(0.24, 0.44, 0.72);
static const float3 sky_bottom = float3(0.75, 0.86, 0.93);

//
// Hit functions.
//

void hit_cube(inout Payload payload, float2 uv);
void hit_floor(inout Payload payload, float2 uv);
void hit_mirror(inout Payload payload, float2 uv);

//
// Ray generation shader.
//

[shader("raygeneration")]
void RayGeneration()
{
    uint2 index = DispatchRaysIndex().xy;
    float2 dims = DispatchRaysDimensions().xy;
    float2 uv = index / dims; // in [0, 1)

    float3 target = float3(
        (uv.x * 2 - 1) * (dims.x / dims.y) * 1.8,
        (1 - uv.y) * 4 - 2 + camera_world.y,
        0);

    RayDesc ray;
    ray.Origin = camera_world;
    ray.Direction = target - camera_world;
    ray.TMin = 0.001;
    ray.TMax = 100;

    Payload payload;
    payload.color = float3(0, 0, 0);
    payload.allow_reflection = true;
    payload.missed = false;

    // https://learn.microsoft.com/en-us/windows/win32/direct3d12/traceray-function
    const uint InstanceInclusionMask = 0xFF; // include all geometry instances, ignore none
    TraceRay(scene_tlas, RAY_FLAG_NONE, InstanceInclusionMask, 0, 0, 0, ray, payload);

    scene_output[index] = float4(payload.color, 1.0);
}

//
// Miss shader.
//

[shader("miss")]
void Miss(inout Payload payload)
{
    float3 ray_dir_world = WorldRayDirection();
    float slope = normalize(ray_dir_world).y;
    float t = saturate(slope * 5 + 0.5);

    payload.color = lerp(sky_bottom, sky_top, t);
    payload.missed = true;
}

//
// Closest-hit shader.
//

[shader("closesthit")]
void ClosestHit(inout Payload payload, BuiltInTriangleIntersectionAttributes attribs)
{
    float2 uv = attribs.barycentrics;

    switch (InstanceID())
    {
    case CUBE_ID:
        hit_cube(payload, uv);
        break;
    case FLOOR_ID:
        hit_floor(payload, uv);
        break;
    case MIRROR_ID:
        hit_mirror(payload, uv);
        break;
    default:
        payload.color = UNKNOWN_ID_HIT_COLOR;
        break;
    }
}

//
// Hit functions implementation.
//

void hit_cube(inout Payload payload, float2 uv)
{
    uint tri = PrimitiveIndex();
    uint face = tri / 2;
    float3 normal_object = (face.xxx % 3 == uint3(0, 1, 2)) * (face < 3 ? -1 : 1);

    // @Todo: @Robustness: https://github.com/graphitemaster/normals_revisited
    // to handle non-uniform scaling, which the following won't do.
    float3 normal_world = normalize(mul(normal_object, (float3x3)ObjectToWorld4x3()));

#if HELLO_CUBE_BARYCENTRICS()
    float3 color = float3(uv, 0);
#else
    float3 color = (uv.x < 0.03 || uv.y < 0.03)
                       ? float3(0.25, 0.25, 0.25)
                       : (abs(normal_object) / 3 + 0.5);
    color *= saturate(dot(normal_world, normalize(light_world))) + 0.33;
#endif

    payload.color = color;
}

void hit_floor(inout Payload payload, float2 uv)
{
    float3 pos_world = WorldRayOrigin() + WorldRayDirection() * RayTCurrent();

    bool2 pattern = frac(pos_world.xz) > 0.5;
    payload.color = (pattern.x ^ pattern.y ? 0.6 : 0.4).xxx;

    RayDesc ray;
    ray.Origin = pos_world;
    ray.Direction = light_world - pos_world;
    ray.TMin = 0.001;
    ray.TMax = 1; // @@

    Payload shadow_payload;
    shadow_payload.color = float3(0, 0, 0);
    shadow_payload.allow_reflection = false;
    shadow_payload.missed = false;
    TraceRay(scene_tlas, RAY_FLAG_NONE, 0xFF, 0, 0, 0, ray, shadow_payload);

    if (!shadow_payload.missed)
    {
        payload.color *= 0.5;
    }
}

void hit_mirror(inout Payload payload, float2 uv)
{
    if (!payload.allow_reflection)
    {
        return;
    }

    float3 pos_world = WorldRayOrigin() + WorldRayDirection() * RayTCurrent();
    float3 normal_world = normalize(mul(Y_UP, (float3x3)ObjectToWorld4x3())); // @Robustness: ditto
    float3 reflected_dir_world = reflect(normalize(WorldRayDirection()), normal_world);

    RayDesc ray;
    ray.Origin = pos_world;
    ray.Direction = reflected_dir_world;
    ray.TMin = 0.001;
    ray.TMax = 100;

    payload.allow_reflection = false; // prevent infinite recursion
    TraceRay(scene_tlas, RAY_FLAG_NONE, 0xFF, 0, 0, 0, ray, payload);
}

