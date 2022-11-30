#include <stdio.h>

int main(int argc, char* argv[]) {
	char buf[23];
	gets(buf);
	printf("%s", buf);
}
