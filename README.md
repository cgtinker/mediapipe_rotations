# Mediapipe Rotations

### Currently in development process.
Future python package to calculate rotation data for Googles Mediapipe Detection results.
The calculations heavly rely on the crate `cgt_math` which is also in development process.

### Setup dev enviroment
Either run the `setup.bash` or setup manually:

```
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
maturin develop
pytest
```

### Usage Example
```
import mediapipe as mp
import mediapipe_rotations as mpr
import cv2

mp_drawing = mp.solutions.drawing_utils
mp_drawing_styles = mp.solutions.drawing_styles
mp_holistic = mp.solutions.holistic


def cvt2array(landmark_list):
    """ converts landmark list to list. """
    return [[landmark.x, landmark.y, landmark.z] for landmark in landmark_list.landmark]


cap = cv2.VideoCapture(0)
with mp_holistic.Holistic(min_detection_confidence=0.5, min_tracking_confidence=0.5) as holistic:
    while cap.isOpened():
        success, image = cap.read()
        if not success:
            print("Ignoring empty camera frame.")
            # If loading a video, use 'break' instead of 'continue'.
            continue

        # To improve performance, optionally mark the image as not writeable to
        # pass by reference.
        image.flags.writeable = False
        image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
        results = holistic.process(image)

        # Get Detection Data
        pose, face, l_hand, r_hand = [], [], [], []

        if results.pose_landmarks:
            pose = cvt2array(results.pose_landmarks)

        if results.face_landmarks:
            face = cvt2array(results.face_landmarks)

        if results.left_hand_landmarks and results.right_hand_landmarks:
            l_hand = cvt2array(results.left_hand_landmarks)
            r_hand = cvt2array(results.right_hand_landmarks)
        elif results.left_hand_landmarks:
            l_hand = cvt2array(results.left_hand_landmarks)
        elif results.right_hand_landmarks:
            r_hand = cvt2array(results.right_hand_landmarks)

        # Calculate rotations
        pose_rotation_quaternion = mpr.pose(pose)
        hand_rotation_quaternionL = mpr.hand(l_hand)
        hand_rotation_quaternionR = mpr.hand(r_hand)
        hands_rotation_quaternion = mpr.hands([l_hand, r_hand])
        face_rotation_quaternion = mpr.face(face)
        holistic_rotation_quaternion = mpr.holistic([pose, face, l_hand, r_hand])

        # Draw landmark annotation on the image.
        image.flags.writeable = True
        image = cv2.cvtColor(image, cv2.COLOR_RGB2BGR)
        mp_drawing.draw_landmarks(
            image,
            results.face_landmarks,
            mp_holistic.FACEMESH_CONTOURS,
            landmark_drawing_spec=None,
            connection_drawing_spec=mp_drawing_styles
                .get_default_face_mesh_contours_style())

        mp_drawing.draw_landmarks(
            image,
            results.pose_landmarks,
            mp_holistic.POSE_CONNECTIONS,
            landmark_drawing_spec=mp_drawing_styles
                .get_default_pose_landmarks_style())

        # Flip the image horizontally for a selfie-view display.
        cv2.imshow('MediaPipe Holistic', cv2.flip(image, 1))
        if cv2.waitKey(5) & 0xFF == 27:
            break
cap.release()

```

### Results
The results come in quaternions.
There are some blank spots in the returned array, mainly for easier indexing.

**Face**
| Idx | Target          | + |
| --- | ----------------| - |
|  0  | head            | + |
|  1  | chin            | + |
|  2  | mouth corner.L  | + |
|  3  | mouth corner.R  | + |

**Hand**
| Idx | Target            | + |
| --- | ----------------- | - |
| 0   | wrist             | + |
| 1   | thumb cmc         | + |
| 2   | thumb mcp         | + |
| 3   | thumb ip          | + |
| 4   | thumb tip         |   |
| 5   | index finger mcp  | + |
| 6   | index finger pip  | + |
| 7   | index finger dip  | + |
| 8   | index finger tip  |   |
| 9   | middle finger mcp | + |
| 10  | middle finger pip | + |
| 11  | middle finger dip | + |
| 12  | middle finger tip |   |
| 13  | ring finger mcp   | + |
| 14  | ring finger pip   | + |
| 15  | ring finger dip   | + |
| 16  | ring finger tip   |   |
| 17  | pinky mcp         | + |
| 18  | pinky pip         | + |
| 19  | pinky dip         | + |
| 20  | pinky tip         |   |

**Pose**
| 0  | nose             | + |
| -- | ---------------- | - |
| 1  | left eye inner   |   |
| 2  | left eye         |   |
| 3  | left eye outer   |   |
| 4  | right eye inner  |   |
| 5  | right eye        |   |
| 6  | right eyerouter  |   |
| 7  | left ear         |   |
| 8  | right ear        |   |
| 9  | mouth left       |   |
| 10 | mouth right      |   |
| 11 | left shoulder    | + |
| 12 | right shoulder   | + |
| 13 | left elbow       | + |
| 14 | right elbow      | + |
| 15 | left wrist       | + |
| 16 | right wrist      | + |
| 17 | left pinky       |   |
| 18 | right pinky      |   |
| 19 | left index       |   |
| 20 | right index      |   |
| 21 | left thumb       |   |
| 22 | right thumb      |   |
| 23 | left hip         | + |
| 24 | right hip        | + |
| 25 | left knee        | + |
| 26 | right knee       | + |
| 27 | left ankle       | + |
| 28 | right ankle      | + |
| 29 | left heel        |   |
| 30 | right heel       |   |
| 31 | left foot index  |   |
| 32 | right foot index |   |
| 33 | hip center       | + |
| 34 | shoulder center  | + |
