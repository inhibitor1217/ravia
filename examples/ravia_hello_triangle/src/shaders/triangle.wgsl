@vertex
fn vs_main(
  @builtin(vertex_index) in_vertex_index: u32
) -> @builtin(position) vec4<f32> {
  let pos = vec4<f32>(
    f32(1 - i32(in_vertex_index)) * 0.5,
    f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5,
    0.0,
    1.0
  );

  return pos;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
  return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
