extern crate cgt_math;
use cgt_math::{Face, Quaternion, Vector3};

const FINGERS: [[usize; 4]; 5] = [
    [1, 2, 3, 4],
    [5, 6, 7, 8],
    [9, 10, 11, 12],
    [13, 14, 15, 16],
    [17, 18, 19, 20],
];
const JOINTS: [[usize; 3]; 3] = [[0, 1, 2], [1, 2, 3], [2, 3, 4]];

pub fn main(hand: &[[f32; 3]; 21], set_origin: bool) {
    let mut data: [Vector3; 22] = [Vector3::ZERO; 22];
    for i in 0..21 {
        data[i] = Vector3::from_array(hand[i]);
    }

    if set_origin {
        set_hand_origin(&mut data);
    }

    calculate_rotations(&data);
}

fn calculate_rotations(data: &[Vector3; 22]) {
    let mut angles: [[f32; 2]; 22] = [[f32::NAN; 2]; 22];
    x_angles(&data, &mut angles);
    println!("{:?}", angles);
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
        data[i].z = tmp.y;
    }
    data[21] = offset;
}

/// Projects finger points on plane, then calculates finger x-angles.
fn x_angles(data: &[Vector3; 22], angles: &mut [[f32; 2]; 22]) {
    for idx in 0..5 {
        // reference finger
        let mut finger: [Vector3; 5] = [Vector3::ZERO; 5];
        finger[0] = data[0];
        for (i, j) in FINGERS[idx].iter().enumerate() {
            finger[i+1] = data[*j];
        }

        // straighten finger
        let plane = Face::from_vecs(finger[0], finger[1], finger[4], [0, 1, 2]);
        for i in 0..5 {
            finger[i] = plane.project(finger[i]);
        }

        // calculate angles
        for (i, joint) in JOINTS.iter().enumerate() {
            let a: Vector3 = finger[joint[1]] - finger[joint[0]];
            let b: Vector3 = finger[joint[2]] - finger[joint[1]];
            angles[FINGERS[idx][i]][0] = a.angle(b);
        }
    }
}
