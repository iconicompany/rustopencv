docker run \
  --name temp \
  -it \
  --cap-add=SYS_PTRACE \
  --security-opt seccomp=unconfined \
  rustopencv $* # bash
echo $?

#docker container inspect temp
docker container cp temp:out.png ./
docker container rm temp
 