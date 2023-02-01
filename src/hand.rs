extern crate cgt_math;
use cgt_math::{Plane, Quaternion, Vector3, Points};

const FINGERS: [[usize; 4]; 5] = [
    [1, 2, 3, 4],
    [5, 6, 7, 8],
    [9, 10, 11, 12],
    [13, 14, 15, 16],
    [17, 18, 19, 20],
];
const JOINTS: [[usize; 3]; 3] = [[0, 1, 2], [1, 2, 3], [2, 3, 4]];


pub fn main(hand: &Vec<[f32; 3]>) -> [Quaternion; 22] {
    let mut data: [Vector3; 22] = [Vector3::ZERO; 22];
    for i in 0..21 {
        data[i] = Vector3::from_array(hand[i]);
    }
    set_hand_origin(&mut data);
    let rotation_data = calculate_rotations(&data);
    return rotation_data;
}

/// Sets hand origin to wrist.
fn set_hand_origin(data: &mut [Vector3; 22]) {
    let offset = data[0];
    for i in 0..22 {
        data[i] -= offset;
    }
    for i in 0..22 {
        let tmp = data[i];
        data[i].x = -tmp.x;
        data[i].y = tmp.z;
        data[i].z = -tmp.y;
    }
    data[21] = offset;
}

fn calculate_rotations(data: &[Vector3; 22]) -> [Quaternion; 22]{
    let mut angles = [[f32::NAN; 2]; 22];
    let mut rotation_data = [Quaternion::NAN; 22];
    x_angles(&data, &mut angles);
    z_angles(&data, &mut angles);
    hand_rotation(&data, &mut rotation_data);
    for (i, xz) in angles.iter().enumerate() {
        if !xz[0].is_nan() {
            rotation_data[i] = Quaternion::from_rotation_x(xz[0]);
        }
        if !xz[1].is_nan() {
            rotation_data[i] *= Quaternion::from_rotation_z(xz[1]);
        }
    }
    return rotation_data;
}

/// Projects finger points on plane, then calculates finger x-angles.
fn x_angles(data: &[Vector3; 22], angles: &mut [[f32; 2]; 22]) {
    for idx in 0..5 {
        // reference finger
        let mut finger: [Vector3; 5] = [Vector3::ZERO; 5];
        finger[0] = data[0];
        for (i, j) in FINGERS[idx].iter().enumerate() {
            finger[i + 1] = data[*j];
        }

        // straighten finger
        let plane = Plane::from_vecs(finger[0], finger[1], finger[4], [0, 1, 2]);
        for i in 0..5 {
            finger[i] = plane.project(finger[i]);
        }

        // calculate angles
        for (i, joint) in JOINTS.iter().enumerate() {
            let joint_tail: Vector3 = finger[joint[1]] - finger[joint[0]];
            let joint_head: Vector3 = finger[joint[2]] - finger[joint[1]];
            angles[FINGERS[idx][i]][0] = joint_tail.angle(joint_head);
        }
    }
}

/// Project finger mcps on a vector between index and pinky mcp.
/// Create circles around the mcps circles facing in the direction of vectors depending on the palm.
/// Searching for the closest point on the circle to the fingers dip and calculate the angle.
/// Thumb gets projected on a plane between thumb mcp, index mcp and wrist to calculate the z-angle.
fn z_angles(data: &[Vector3; 22], angles: &mut [[f32; 2]; 22]) {
    // approx thumb z-angle
    let thumb_plane = Plane::from_vecs(data[0], data[1], data[5], [0, 1, 2]);
    let mut thumb = [Vector3::ZERO; 3];
    for (i, vector) in [data[1], data[5], data[2]].iter().enumerate() {
        thumb[i] = thumb_plane.project(*vector);
    }
    let thumb_joint_tail = thumb[0] - thumb[1];
    let thumb_joint_head = thumb[0] - thumb[2];
    angles[1][1] = thumb_joint_tail.angle(thumb_joint_head);

    // approx other finger z-angles
    // get references at first
    let tangent = data[17] - data[5]; // knuckle line
    let mut mcps = [Vector3::ZERO; 4];
    let mut pips = [Vector3::ZERO; 4];
    let mut dists: [f32; 4] = [0.0; 4];
    for (i, finger) in FINGERS.iter().skip(1).enumerate() {
        // TODO: check if adding the index mcp location to the projection is necessary
        let (p, a, b) = (data[finger[0]], data[5], data[17]);
        let ap = p - a;
        let ab = b - a;
        mcps[i] = a + ap.project(ab);
        pips[i] = data[finger[2]];
        dists[i] = mcps[i].distance_to(pips[i]);
    }

    // circle props based on hand
    const N: usize = 6;
    let pinky_vec = data[17] - data[0];
    let thumb_vec = data[5] - data[1];
    let dir_vecs: [Vector3; 4] = [pinky_vec, pinky_vec, thumb_vec, thumb_vec];

    for i in 0..4 {
        // find the closest point from circle to pip
        let normal = tangent.cross(dir_vecs[i]);
        let circle = Points::circle_from_uv(mcps[i], dir_vecs[i].normalize(), normal.normalize(),  dists[i], N);
        let closest = circle.closest_to_idx(pips[i]);

        // calc abs z-angle
        let mcp_pip = pips[i] - mcps[i];
        let mcp_closest = circle[closest] - mcps[i];
        let mut angle = mcp_pip.angle(mcp_closest);

        // check if pip is infront or behind circle
        let mut expanded_circle: Vec<Vector3> = Vec::with_capacity(N*3);
        for _ in 0..3 {
            for x in circle.iter() {
                expanded_circle.push(*x);
            }
        }
        let a = expanded_circle[N+closest + 6];
        let b = expanded_circle[N+closest - 6];

        // negative angle based on pip position
        let plane = Plane::from_vecs(a, circle[closest], b, [0,1,2]);
        if plane.distance(pips[i]) < 0.0f32 {
            angle *= -1.0;
        }
        angles[FINGERS[i+1][0]][1] = angle;
    }
}

/// Calculate wrist rotation
fn hand_rotation(data: &[Vector3; 22], rotation_data: &mut [Quaternion; 22]) {
    let normal = (data[5]-data[1]).normalize();
    let binormal = (data[13]-data[5]).normalize();
    let tangent = (binormal.cross(normal)).normalize();
    rotation_data[0] = Quaternion::from_rotation_axes(tangent, normal, binormal);
}
