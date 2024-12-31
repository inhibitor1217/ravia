struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) uv: vec2<f32>,
};

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0) uv: vec2<f32>,
};

struct CameraUniform {
  projection: mat4x4<f32>,
};

struct TransformUniform {
  transform: mat4x4<f32>,
  transform_inv: mat4x4<f32>,
};

@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;

@group(1) @binding(0) var<uniform> camera: CameraUniform;

@group(2) @binding(0) var<uniform> camera_transform: TransformUniform;

@group(3) @binding(0) var<uniform> model_transform: TransformUniform;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
  var out: VertexOutput;
  out.position = camera.projection * camera_transform.transform_inv * model_transform.transform * vec4<f32>(in.position, 1.0);
  out.uv = in.uv;
  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return textureSample(tex, tex_sampler, in.uv);
}
