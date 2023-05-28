struct CustomMaterial {
    color: vec4<f32>,
    }

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var color_texture: texture_2d<f32>;
@group(1) @binding(2)
var color_sampler: sampler;

@fragment
fn fragment(
) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0); // Red color
}