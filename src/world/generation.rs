use noise::{NoiseFn, Perlin};
use bevy::{asset::RenderAssetUsages, mesh::*, prelude::*};

pub fn generate_terrain_mesh(
    width: u32,
    depth: u32,
    noise_scale: f64,
    height_scale: f32,
    seed: u32,
) -> Mesh {
    let perlin = Perlin::new(seed);
    let bottom_y = -5000.0;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let height_at = |x: u32, z: u32| -> f32 {
        octave_noise(&perlin, x as f64 * noise_scale, z as f64 * noise_scale, 6)
            * height_scale
    };

    // -- Top Face (playable terrain) --
    let mut top_heights = vec![0.0f32; ((width + 1) * (depth + 1)) as usize];

    for z in 0..=depth {
        for x in 0..=width {
            let y = height_at(x, z);
            top_heights[(z * (width + 1) + x) as usize] = y;

            positions.push([x as f32, y, z as f32]);
            normals.push([0.0, 1.0, 0.0]); // recomputed below
            uvs.push([x as f32 / width as f32, z as f32 / depth as f32]);
        }
    }

    for z in 0..depth {
        for x in 0..width {
            let i = z * (width + 1) + x;
            indices.extend_from_slice(&[i, i + width + 1, i + 1]);
            indices.extend_from_slice(&[i + 1, i + width + 1, i + width + 2]);
        }
    }

    let top_normals = compute_normals(&positions, &indices);
    for (i, n) in top_normals.iter().enumerate() {
        normals[i] = *n;
    }

    // -- Side walls --

    // Front (z = 0, normal = -Z)
    for x in 0..width {
        let top_l = top_heights[(0 * (width + 1) + x) as usize];
        let top_r = top_heights[(0 * (width + 1) + x + 1) as usize];
        emit_wall_quad(
            &mut positions, &mut normals, &mut uvs, &mut indices,
            [x as f32, top_l, 0.0],
            [x as f32 + 1.0, top_r, 0.0],
            bottom_y,
            [0.0, 0.0, -1.0],
            x, width,
        );
    }

    // Back (z = depth, normal = +Z)
    for x in 0..width {
        let top_l = top_heights[(depth * (width + 1) + x) as usize];
        let top_r = top_heights[(depth * (width + 1) + x + 1) as usize];
        emit_wall_quad(
            &mut positions, &mut normals, &mut uvs, &mut indices,
            [x as f32 + 1.0, top_r, depth as f32],
            [x as f32, top_l, depth as f32],
            bottom_y,
            [0.0, 0.0, 1.0],
            x, width,
        );
    }

    // Left (x = 0, normal = -X)
    for z in 0..depth {
        let top_l = top_heights[(z * (width + 1)) as usize];
        let top_r = top_heights[((z + 1) * (width + 1)) as usize];
        emit_wall_quad(
            &mut positions, &mut normals, &mut uvs, &mut indices,
            [0.0, top_r, z as f32 + 1.0],
            [0.0, top_l, z as f32],
            bottom_y,
            [-1.0, 0.0, 0.0],
            z, depth,
        );
    }

    // Right (x = width, normal = +X)
    for z in 0..depth {
        let top_l = top_heights[(z * (width + 1) + width) as usize];
        let top_r = top_heights[((z + 1) * (width + 1) + width) as usize];
        emit_wall_quad(
            &mut positions, &mut normals, &mut uvs, &mut indices,
            [width as f32, top_l, z as f32],
            [width as f32, top_r, z as f32 + 1.0],
            bottom_y,
            [1.0, 0.0, 0.0],
            z, depth,
        );
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

// Emits a single wall quad (two triangles) with a flat normal
fn emit_wall_quad(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
    top_left: [f32; 3],
    top_right: [f32; 3],
    bottom_y: f32,
    normal: [f32; 3],
    segment: u32,
    total: u32,
) {
    let base = positions.len() as u32;

    let u_l = segment as f32 / total as f32;
    let u_r = (segment + 1) as f32 / total as f32;

    positions.extend_from_slice(&[
        top_left,
        top_right,
        [top_right[0], bottom_y, top_right[2]],
        [top_left[0],  bottom_y, top_left[2] ],
    ]);
    normals.extend_from_slice(&[normal; 4]);
    uvs.extend_from_slice(&[
        [u_l, 0.0],
        [u_r, 0.0],
        [u_r, 1.0],
        [u_l, 1.0],
    ]);

    indices.extend_from_slice(&[
        base, base + 1, base + 2,
        base, base + 2, base + 3,
    ]);
}

fn compute_normals(positions: &[[f32; 3]], indices: &[u32]) -> Vec<[f32; 3]> {
    let mut normals = vec![Vec3::ZERO; positions.len()];

    for tri in indices.chunks(3) {
        let (a, b, c) = (tri[0] as usize, tri[1] as usize, tri[2] as usize);
        let v0 = Vec3::from(positions[a]);
        let v1 = Vec3::from(positions[b]);
        let v2 = Vec3::from(positions[c]);

        let face_normal = (v1 - v0).cross(v2 - v0);
        normals[a] += face_normal;
        normals[b] += face_normal;
        normals[c] += face_normal;
    }

    normals.iter().map(|n| n.normalize().into()).collect()
}

fn octave_noise(perlin: &Perlin, x: f64, z: f64, octaves: u32) -> f32 {
    let mut value = 0.0f64;
    let mut amplitude = 1.0f64;
    let mut frequency = 1.0f64;
    let mut max_value = 0.0f64;

    for _ in 0..octaves {
        value += perlin.get([x * frequency, z * frequency]) * amplitude;
        max_value += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    (value / max_value) as f32
}
