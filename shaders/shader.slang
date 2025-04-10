struct VertexInput
{
    float2 position;
    float3 color;
}

struct VertexOutput
{
    float4 position : SV_POSITION;
    float3 color : TEXCOORD0;
}

[shader("vertex")]
func vs_main(VertexInput input)->VertexOutput
{
    VertexOutput output;
    output.position = float4(input.position, 0.0, 1.0);
    output.color = input.color;
    return output;
}

struct FragmentOutput
{
    float4 color : SV_TARGET;
}

[shader("fragment")]
func fs_main(VertexOutput input)->FragmentOutput
{
    FragmentOutput output;
    output.color = float4(input.color, 1.0);
    return output;
}
