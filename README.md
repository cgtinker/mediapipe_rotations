# Mediapipe Rotations

Calculates rotation data for Googles Mediapipe Detection data.

### Setup dev enviroment
```
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
maturin develop
python3 pybindtest
```

Python bindings to calculate rotations for mediapipe detection results.


### Usage Example
```
import mediapipe as mp
import mediapipe_rotations as mpr
import cv2


mp_drawing = mp.solutions.drawing_utils
mp_drawing_styles = mp.solutions.drawing_styles
mp_holistic = mp.solutions.holistic

def cvt2array(self, landmark_list) -> List[List[float]]:
  """ converts landmark list to list. """
  return [[landmark.x, landmark.y, landmark.z] for landmark in landmark_list.landmark]


cap = cv2.VideoCapture(0)
with mp_holistic.Holistic(min_detection_confidence=0.5,min_tracking_confidence=0.5) as holistic:
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
    
    # Calculate rotations
    pose, face, l_hand, r_hand = [], [], [], []
    
    # Pose Example
    if results.pose_landmarks:
      pose = cvt2array(mp_res.pose_landmarks)
      pose_rotation_quaternion = mpr.pose(pose)
    
    # Face Example
    if mp_res.face_landmarks:
      face = cvt2array(mp_res.face_landmarks)
      face_rotation_quaternion = mpr.face(face)
      
    # Hand Example
    if mp_res.left_hand_landmarks and mp_res.right_hand_landmarks:
      l_hand = cvt2array(mp_res.left_hand_landmarks)
      r_hand = cvt2array(mp_res.right_hand_landmarks)
      hands_rotation_quaternion = mpr.hands([l_hand, r_hand])
    elif mp_res.left_hand_landmarks:
      l_hand = cvt2array(mp_res.left_hand_landmarks)
      l_hand_rotation_quaternion = mpr.hand(l_hand)
    elif mp_res.right_hand_landmarks:
      r_hand = cvt2array(mp_res.right_hand_landmarks)
      r_hand_rotation_quaternion = mpr.hand(r_hand)
   
    # Holistic Example (not implemented yet)
    holistic_rotation_quaternion = mpr.holistic(pose, face, [l_hand, r_hand])

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
