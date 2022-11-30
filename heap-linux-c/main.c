#include <stdlib.h>
#include <stdio.h>

int main(int argc, char *argv[]) {
	int *foo = malloc(1035);
	*foo = 123456;
	*(foo+8) = 1111;
	*(foo+16) = 2222;
	for(int i=0; i<128; i+=8) {
		printf("%p \t %d\n", (foo+i), *(foo+i));
	}
	printf("----------\n");
	free(foo);
	for(int i=0; i<128; i+=8) {
		printf("%p \t %d\n", (foo+i), *(foo+i));
	}
	printf("----------\n");
	int *foo2 = malloc(1035);
	*foo2 = 121212;
	// *(foo2+8) = 22222;
	for(int i=0; i<128; i+=8) {
		printf("%p \t %d\n", (foo2+i), *(foo2+i));
	}
}
