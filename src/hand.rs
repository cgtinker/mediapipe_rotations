extern crate cgt_math;
use cgt_math::{Face, Vector3, Quaternion};


pub fn main(hand: &[[f32; 3]; 21], set_origin: bool) {
    let mut data: [Vector3; 22] = [Vector::ZERO, 22];
    for i in 0..21 {
        data[i] = Vector3::from_array(hand[i]);
    }
    if set_origin {
        set_hand_origin(&data);
    }
}

fn set_hand_origin(data: &mut [Vector3; 22]) {
    let offset = data[0];
    for i in 0..22 {
        data[i] -= offset;
    }
    data[21] = offset;
}
