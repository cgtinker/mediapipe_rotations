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


pub fn main(hand: &[[f32; 3]]) -> [Quaternion; 21] {
    let mut data = to_vectors(hand);
    set_hand_origin(&mut data);

    let mut angles = [[f32::NAN; 2]; 21];
    x_angles(&data, &mut angles);
    z_angles(&data, &mut angles);

    let mut rotation_data = [Quaternion::NAN; 21];
    angles_to_quaternions(&angles, &mut rotation_data);
    hand_rotation(&data, &mut rotation_data);

    return rotation_data;
}

/// Converts data to Vector3s.
fn to_vectors(hand: &[[f32; 3]]) -> [Vector3; 21] {
    let mut data: [Vector3; 21] = [Vector3::ZERO; 21];
    for i in 0..21 {
        data[i] = Vector3::from_array(hand[i]);
    }
    return data;
}

/// Sets hand origin to wrist.
fn set_hand_origin(data: &mut [Vector3; 21]) {
    let offset = data[0];
    for i in 0..21 {
        data[i] -= offset;
        let tmp = data[i];
        data[i].x = -tmp.x;
        data[i].y = tmp.z;
        data[i].z = -tmp.y;
    }
}

/// Attempts to convert finger angles to quaternions.
fn angles_to_quaternions(angles: &[[f32;2]; 21], rotation_data: &mut [Quaternion; 21]) {
    for (i, xz) in angles.iter().enumerate() {
        if !xz[0].is_nan() {
            rotation_data[i] = Quaternion::from_rotation_x(xz[0]);
        }
        if !xz[1].is_nan() {
            rotation_data[i] *= Quaternion::from_rotation_z(xz[1]);
        }
    }
}

/// Projects finger points on plane, then calculates finger x-angles.
fn x_angles(data: &[Vector3; 21], angles: &mut [[f32; 2]; 21]) {
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
fn z_angles(data: &[Vector3; 21], angles: &mut [[f32; 2]; 21]) {
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
fn hand_rotation(data: &[Vector3; 21], rotation_data: &mut [Quaternion; 21]) {
    let normal = (data[5]-data[1]).normalize();
    let binormal = (data[13]-data[5]).normalize();
    let tangent = (binormal.cross(normal)).normalize();
    rotation_data[0] = Quaternion::from_rotation_axes(tangent, normal, binormal);
}

#[cfg(test)]
mod test {
    use cgt_math::{Quaternion, Vector3};
    #[test]
    fn impl_test() {
        let hand_data = [[-0.012344579212367535, 0.07004635035991669, 0.020521901547908783], [0.018071463331580162, 0.047368425875902176, 0.010523390956223011], [0.03255487233400345, 0.016385573893785477, -0.0011732536368072033], [0.037621572613716125, -0.017625989392399788, -0.013580389320850372], [0.043106138706207275, -0.05177343264222145, -0.017558827996253967], [0.024736206978559494, -0.006148995831608772, 0.0019370221998542547], [0.026668652892112732, -0.03547884523868561, -0.006494760047644377], [0.02291758358478546, -0.05428066849708557, -0.011178224347531796], [0.025358645245432854, -0.07094687223434448, -0.03112722560763359], [0.0005194954574108124, -0.0025673473719507456, 0.005111261270940304], [0.003210199996829033, -0.03972770646214485, -0.004665873944759369], [-0.002873774617910385, -0.06354730576276779, -0.018868273124098778], [0.004980511963367462, -0.08323581516742706, -0.030609922483563423], [-0.01502157561480999, 0.0035259551368653774, -0.0005771743599325418], [-0.018982525914907455, -0.03117505833506584, -0.007334231864660978], [-0.01772765815258026, -0.05211472511291504, -0.014353149570524693], [-0.013936810195446014, -0.07475992292165756, -0.028724966570734978], [-0.03519390895962715, 0.01385025680065155, -0.0037085190415382385], [-0.03892548382282257, -0.008683949708938599, -0.00501153664663434], [-0.03714082017540932, -0.02858530357480049, -0.010903152637183666], [-0.03820459172129631, -0.048742808401584625, -0.025562860071659088]];
        let mut data: [Vector3; 21] = super::to_vectors(&hand_data);

        // check if reseting pose works
        super::set_hand_origin(&mut data);
        assert_eq!(data[0], Vector3::ZERO);
        assert_ne!(data[5], Vector3::ZERO);

        let mut angles = [[f32::NAN; 2]; 21];
        super::x_angles(&data, &mut angles);
        super::z_angles(&data, &mut angles);

        // check if rotations have been calculated
        let mut rotation_data = [Quaternion::NAN; 21];
        super::angles_to_quaternions(&angles, &mut rotation_data);
        super::hand_rotation(&data, &mut rotation_data);

        for finger in super::FINGERS {
            for i in 0..3 {
                assert!(rotation_data[finger[i]].is_finite());
            }
        }
    }
}

