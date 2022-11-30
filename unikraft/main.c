#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void __attribute__ ((noinline)) call_target() {
	printf("Called target function!\n");
}


int main(int argc, char *argv[]) {
	printf("----- W^X ------\n");
	// Test code segment write permission
	printf("[info] Print unmodified function:\n");
	printf("func: %lx @ %p\n", *((long*)call_target), &call_target);
	printf("[info] Execute unmodified function:\n");	
	call_target();
	printf("[info] Attempting to overwrite function code with 'ret':\n");
	// Overwrite call_target() function with a simple return statement
	// thus, calling the call_target() function will not print anything
	*((long*)call_target) = 0xccccccc3;
	printf("[info] Written!\n");
	printf("[info] Print modified function:\n");
	printf("func: %lx @ %p\n", *((long*)call_target), &call_target);
	printf("[info] Execute modified function:\n");	
	call_target();
	printf("[info] Done!\n");
}


