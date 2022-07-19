docker run \
  -it \
  --rm \
  --cap-add=SYS_PTRACE \
  --security-opt seccomp=unconfined \
  rustopencv $* # bash

 