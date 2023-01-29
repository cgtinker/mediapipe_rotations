extern crate cgt_math;
use cgt_math::{Face, Quaternion, Vector3};

/// Calculates limb fk chain, hip, shoulder and face rotation.
/// May uses hip center as pivot.
pub fn main(pose: &[[f32; 3]; 33], set_origin: bool) -> ([Vector3; 36], [Quaternion; 36]) {
    let mut data: [Vector3; 36] = [Vector3::ZERO; 36];
    for i in 0..33 {
        data[i] = Vector3::from_array(pose[i]);
    }

    data[34] = (data[11] + data[12]) / 2.0; // shoulder center location
    data[33] = (data[23] + data[24]) / 2.0; // hip center location
    if set_origin {
        set_pose_origin(&mut data);
    }

    let rotation_data = calc_rotation_data(&data);
    return (data, rotation_data);
}

/// Uses hip center as pivot.
fn set_pose_origin(data: &mut [Vector3; 36]) {
    let offset = data[33];
    for i in 0..36 {
        data[i] -= offset;
    }
    data[35] = offset;
}

/// Calcute rotation data.
fn calc_rotation_data(data: &[Vector3; 36]) -> [Quaternion; 36] {
    let mut rotation_data: [Quaternion; 36] = [Quaternion::NAN; 36];
    shoulder_rotation(&data, &mut rotation_data);
    torso_rotation(&data, &mut rotation_data);
    limb_rotations(&data, &mut rotation_data);
    foot_rotations(&data, &mut rotation_data);
    return rotation_data;
}

/// Calculates torso rotation.
fn torso_rotation(data: &[Vector3; 36], rotation_data: &mut [Quaternion; 36]) {
    let normal: Vector3 = Face::from_vecs(data[23], data[24], data[34], [0, 1, 2]).normal(); // left hip, right hip, shoulder center
    let tangent: Vector3 = data[24] - data[33]; // right hip, center hip
    let binormal: Vector3 = data[34] - data[33]; // hip center, shoulder center
    rotation_data[33] = Quaternion::from_rotation_axes(
        tangent.normalize(),
        normal.normalize(),
        binormal.normalize(),
    );
}

/// Calculates shoulder rotation.
/// TODO: Check if results match expectations.
fn shoulder_rotation(data: &[Vector3; 36], rotation_data: &mut [Quaternion; 36]) {
    // As the torso rotation usually is used to rotate the rig,
    // the torso rotation got to be substracted from the hip rotation
    let shoulder_rot =
        Quaternion::from_vec_to_track_quat((data[12] - data[34]).normalize().neg(), 2, 1); // rotation from center to right shoulder
    let hip_rot = Quaternion::from_vec_to_track_quat((data[24] - data[33]).normalize().neg(), 2, 1); // rotation from center to right hip
    rotation_data[34] = shoulder_rot - hip_rot;
}

/// Calculates fk chain rotations.
fn calc_limb_chain_rotations(data: &[Vector3; 4]) -> [Quaternion; 3] {
    let mut arr: [Quaternion; 3] = [Quaternion::IDENTITY; 3];
    for i in 1..4 {
        arr[i - 1] =
            Quaternion::from_vec_to_track_quat((data[i - 1] - data[i]).normalize().neg(), 4, 2);
    }
    return arr;
}

/// Calculates arm and leg rotations.
fn limb_rotations(data: &[Vector3; 36], rotation_data: &mut [Quaternion; 36]) {
    let l_leg_rots: [Quaternion; 3] =
        calc_limb_chain_rotations(&[data[23], data[25], data[27], data[31]]);
    rotation_data[23] = l_leg_rots[0]; // left hip
    rotation_data[25] = l_leg_rots[1]; // left knee

    let r_leg_rots: [Quaternion; 3] =
        calc_limb_chain_rotations(&[data[24], data[26], data[28], data[32]]);
    rotation_data[24] = r_leg_rots[0]; // right hip
    rotation_data[26] = r_leg_rots[1]; // right knee

    let l_arm_rots: [Quaternion; 3] =
        calc_limb_chain_rotations(&[data[11], data[13], data[15], data[19]]);
    rotation_data[11] = l_arm_rots[0]; // left shoulder
    rotation_data[13] = l_arm_rots[1]; // left elbow
    rotation_data[15] = l_arm_rots[2]; // left hand

    let r_arm_rots: [Quaternion; 3] =
        calc_limb_chain_rotations(&[data[12], data[14], data[16], data[20]]);
    rotation_data[11] = r_arm_rots[0]; // right shoulder
    rotation_data[13] = r_arm_rots[1]; // right elbow
    rotation_data[15] = r_arm_rots[2]; // right hand
}

/// Calculates foot rotation.
/// MPs knee, foot_index and heel usually form a triangle.
fn foot_rotations(data: &[Vector3; 36], rotation_data: &mut [Quaternion; 36]) {
    // left
    let tangent = Face::from_vecs(data[25], data[27], data[31], [0, 1, 2]).normal();
    let binormal = data[25] - data[31];
    let normal = data[27] - data[31];
    rotation_data[27] = Quaternion::from_rotation_axes(
        tangent.normalize(),
        normal.normalize(),
        binormal.normalize(),
    );

    // right
    let tangent = Face::from_vecs(data[26], data[28], data[32], [0, 1, 2]).normal();
    let binormal = data[26] - data[32];
    let normal = data[28] - data[32];
    rotation_data[28] = Quaternion::from_rotation_axes(
        tangent.normalize(),
        normal.normalize(),
        binormal.normalize(),
    );
}
