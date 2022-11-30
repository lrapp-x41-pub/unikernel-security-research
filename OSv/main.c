#include <stdio.h>

void print_stack(long *marker_addr) {
	for (int i = 0; i <= 6; i++) {
		printf("%p: \t 0x%lx\n", (marker_addr+i), *(marker_addr+i));
	}
}

void __attribute__ ((noinline)) myfunc() {
	char var[16] = "CCCC";
	long marker = 0x6666666666666666;
	print_stack(&marker);
	printf("Canary: 0x%lx @ %p\n", *((&marker)+0x4), ((&marker)+0x4));
	printf("Input plz\n");
	gets(&var);
	print_stack(&marker);
}

int main(int argc, char *argv[])
{
	printf("main(): 0x%p\n", &main);
	myfunc();
	return 0;
}
