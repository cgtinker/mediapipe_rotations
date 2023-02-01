extern crate cgt_math;
use cgt_math::{Plane, Quaternion, Vector3};

/// Calculates limb fk chain, hip, shoulder and face rotation.
/// May uses hip center as pivot.
pub fn main(pose: &[[f32; 3]]) -> [Quaternion; 36] {
    let mut data = to_vectors(pose);
    set_pose_origin(&mut data);

    let mut rotation_data: [Quaternion; 36] = [Quaternion::NAN; 36];
    calc_rotation_data(&data, &mut rotation_data);
    return rotation_data;
}

/// Convert data to vectors
fn to_vectors(pose: &[[f32; 3]]) -> [Vector3; 36] {
    let mut data: [Vector3; 36] = [Vector3::ZERO; 36];
    for i in 0..33 {
        data[i] = Vector3::from_array(pose[i]);
    }
    return data;
}

/// Uses hip center as pivot, also add shoulder center.
fn set_pose_origin(data: &mut [Vector3; 36]) {
    data[34] = (data[11] + data[12]) / 2.0; // shoulder center location
    data[33] = (data[23] + data[24]) / 2.0; // hip center location
    let offset = data[33];
    for vec in data.iter_mut() {
        *vec -= offset;
    }
    data[35] = offset;
}

/// Calcute rotation data.
fn calc_rotation_data(data: &[Vector3; 36], rotation_data: &mut [Quaternion; 36]) {
    shoulder_rotation(&data, rotation_data);
    torso_rotation(&data, rotation_data);
    limb_rotations(&data, rotation_data);
    foot_rotations(&data, rotation_data);
}

/// Calculates torso rotation.
fn torso_rotation(data: &[Vector3; 36], rotation_data: &mut [Quaternion; 36]) {
    let normal: Vector3 = Plane::from_vecs(data[23], data[24], data[34], [0, 1, 2]).normal(); // left hip, right hip, shoulder center
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
    rotation_data[12] = r_arm_rots[0]; // right shoulder
    rotation_data[14] = r_arm_rots[1]; // right elbow
    rotation_data[16] = r_arm_rots[2]; // right hand
}

/// Calculates foot rotation.
/// MPs knee, foot_index and heel usually form a triangle.
fn foot_rotations(data: &[Vector3; 36], rotation_data: &mut [Quaternion; 36]) {
    // left
    let tangent = Plane::from_vecs(data[25], data[27], data[31], [0, 1, 2]).normal();
    let binormal = data[25] - data[31];
    let normal = data[27] - data[31];
    rotation_data[27] = Quaternion::from_rotation_axes(
        tangent.normalize(),
        normal.normalize(),
        binormal.normalize(),
    );

    // right
    let tangent = Plane::from_vecs(data[26], data[28], data[32], [0, 1, 2]).normal();
    let binormal = data[26] - data[32];
    let normal = data[28] - data[32];
    rotation_data[28] = Quaternion::from_rotation_axes(
        tangent.normalize(),
        normal.normalize(),
        binormal.normalize(),
    );
}

#[cfg(test)]
mod test {
    use cgt_math::{Quaternion, Vector3};
    #[test]
    fn impl_test() {
        let pose_data: [[f32; 3]; 33] = [[0.01683431677520275, -0.5720066428184509, -0.1913946270942688], [0.024014215916395187, -0.6067853569984436, -0.17255261540412903], [0.024569006636738777, -0.6086560487747192, -0.17198845744132996], [0.024781377986073494, -0.6091530919075012, -0.17249786853790283], [-0.004876093938946724, -0.6023746728897095, -0.1696224808692932], [-0.004200221970677376, -0.603059709072113, -0.17153173685073853], [-0.005072474479675293, -0.6038256883621216, -0.16956579685211182], [0.0735815092921257, -0.5825239419937134, -0.07638362050056458], [-0.05969294160604477, -0.5789850950241089, -0.07614320516586304], [0.038081832230091095, -0.5480473637580872, -0.16053661704063416], [-0.001756865531206131, -0.5424045324325562, -0.15814310312271118], [0.16237878799438477, -0.4207139313220978, -0.01245066523551941], [-0.15345652401447296, -0.42451488971710205, -0.041541457176208496], [0.23344291746616364, -0.21467289328575134, -0.017463013529777527], [-0.22142092883586884, -0.2176237255334854, -0.06124362349510193], [0.2626833915710449, -0.03526216000318527, -0.10620015859603882], [-0.2471379190683365, -0.06586073338985443, -0.17158746719360352], [0.27902939915657043, 0.031674716621637344, -0.12129878997802734], [-0.23449411988258362, -0.008173542097210884, -0.1931554079055786], [0.2527311444282532, 0.013788735494017601, -0.15270227193832397], [-0.20015230774879456, -0.01838053949177265, -0.21121203899383545], [0.2455325573682785, -0.029286660254001617, -0.11769473552703857], [-0.22591270506381989, -0.054641157388687134, -0.18510574102401733], [0.1503724455833435, 0.005249669309705496, 0.006746768951416016], [-0.1512945592403412, -0.030116502195596695, 1.430511474609375e-06], [0.09783992916345596, -0.04433642327785492, -0.14216428995132446], [-0.10689786821603775, -0.3214568793773651, -0.2423054575920105], [0.2054353803396225, 0.21683450043201447, -0.05112290382385254], [-0.06183558329939842, -0.017422612756490707, -0.23209118843078613], [0.20023104548454285, 0.28065335750579834, -0.011745452880859375], [-0.08652661740779877, 0.045679036527872086, -0.1549696922302246], [0.3208251893520355, -0.07020407915115356, -0.3819558620452881], [-0.056145183742046356, -0.23949319124221802, -0.5386220812797546]];
        let mut data: [Vector3; 36] = super::to_vectors(&pose_data);

        // check if reseting pose works
        super::set_pose_origin(&mut data);
        assert_eq!(data[33], Vector3::ZERO);
        assert_ne!(data[35], Vector3::ZERO);

        // check if rotations have been calculated
        let mut rotation_data: [Quaternion; 36] = [Quaternion::NAN; 36];
        super::calc_rotation_data(&data, &mut rotation_data);
        for idx in [33, 34, // torso & hips
            11, 12, 13, 14, 15, 16, // arms & finally legs
            23, 24, 25, 26, 27, 28,] {
            assert!(rotation_data[idx].is_finite());
        }
    }
}
