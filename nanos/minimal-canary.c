#include <stdio.h>

void __attribute__ ((noinline)) print_canary() {
	long marker = 0xACDC;
	printf("Canary: 0x%lx\n", *((&marker)+0x1));
}

int main(int argc, char* argv[]) {
	print_canary();
}
