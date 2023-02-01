import mediapipe_rotations as mpr
import json
from pathlib import Path
from math import isnan


pose_data = None
hand_data = None
face_data = None
with open(str(Path(__file__).parent / "mp_data.json")) as fp:
    json_data = json.load(fp)
    pose_data = json_data["pose"]
    hand_data = json_data["hand"]
    face_data = json_data["face"]

def check_pose(r):
    assert len(r) == 36, "Pose data should be length of 36, input length has been {}".format(len(r))
    for idx in [33, 34, 11, 12, 13, 14, 15, 16, 23, 24, 25, 26, 27, 28]:
        assert len(r[idx]) == 4
        assert all([True if not isnan(x) else False for x in r[idx]])

def check_hand(r):
    assert len(r) == 21, "Hand data should be length of 21, input length has been {}".format(len(r))
    for idx in [0, 1, 2, 3, 5, 6, 7, 9, 10, 11, 13, 14, 15, 17, 18, 19]:
        assert len(r[idx]) == 4
        assert all([True if not isnan(x) else False  for x in r[idx]])

def check_face(r):
    assert len(r) == 4, "Face data should be length of 4, input length has been {}".format(len(r))
    for x in r:
        assert (all([not isnan(i) for i in x]))

def check_is_nan(r):
    for x in r:
        assert (all([isnan(i) for i in x]))

def test_pose():
    r = mpr.pose(pose_data)
    check_pose(r)
    r = mpr.pose([])
    check_is_nan(r)

def test_hand():
    r = mpr.hand(hand_data)

    # test hands
    hands_data = [hand_data, hand_data]
    l, r = mpr.hands(hands_data)
    check_hand(l)
    check_hand(r)

    # test hands with empty array
    hands_data = [hand_data, []]
    l, r = mpr.hands([hand_data, []])
    check_hand(l)
    check_is_nan(r)
    l, r = mpr.hands([[], []])
    check_is_nan(l)
    check_is_nan(r)

def test_face():
    r = mpr.face(face_data)
    check_face(r)

    # test face nan
    r = mpr.face([])
    check_is_nan(r)

def test_holistic():
    holistic_data = [pose_data, face_data, hand_data, hand_data]
    r = mpr.holistic(holistic_data)
    pose_r, face_r, hand_l_r, hand_r_r = r
    check_pose(pose_r)
    check_face(face_r)
    check_hand(hand_l_r)
    check_hand(hand_r_r)

    # test holistic nan
    holistic_data = [[], [], [], []]
    r = mpr.holistic(holistic_data)
    assert len(r) == 4
    for x in r:
        check_is_nan(x)
