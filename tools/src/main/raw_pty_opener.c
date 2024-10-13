#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <fcntl.h>
#include <stdbool.h>

void read_till_avail_with_write(int src_fd, int target_fd) {
  int buf[1024];
  int read_bytes;
  // read till you have the input in src_fd
  while((read_bytes = read(src_fd, buf, 1024)) > 0) {
    printf("[DEBUG] transfer: %d\n", read_bytes);
    write(target_fd, buf, read_bytes);
  }

  // flush changes to the fd
  fsync(target_fd);
}

int main(int argc, char **argv) {
  if(argc != 2) {
    perror("Only one argument is required specifying the psuedo-terminal to be opened, like: /dev/pts/0");
    exit(1);
  }

  char* pts_path = argv[1];
  int fd = open(pts_path, O_RDWR);

	dup2(fd, STDIN_FILENO);
	dup2(fd, STDOUT_FILENO);
	dup2(fd, STDERR_FILENO);

	char *empty_env_args[] = {(char*)0};
	execve("/bin/zsh", empty_env_args, empty_env_args);

  return 0;
}
