#include <stdio.h>

void __attribute__ ((noinline)) func() {
	printf("Hello, world from C!");
}

int main() {
	func();
}
