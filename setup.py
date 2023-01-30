from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="mediapipe_rotations",
    version="0.0",
    rust_extensions=[RustExtension("mediapipe_rotations.mediapipe_rotations", binding=Binding.PyO3)],
    packages=["mediapipe_rotations"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
