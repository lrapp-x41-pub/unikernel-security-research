#include <stdio.h>
#include <string.h>
#include <stdint.h>

__attribute__((noinline))
void target_func() {
	printf("Called target function!\n");
	return;
}

__attribute__((noinline))
void other_func() {
	target_func();
	return;
}

int main() {
	printf("Main: %p\n", &main);
	printf("===.text===\n");
	printf("other_func: 0x%x at %p \n", *((int*) &other_func), &other_func);
	printf("Calling unmodified other_func\n");
	other_func();
	printf("Returend from unmodified other_func\n");
 	*((int*) &other_func) = 0xccccccc3;
	printf("other_func: 0x%x at %p \n", *((int*) &other_func), &other_func);
	printf("Calling modified other_func\n");
	other_func();
	printf("Returend from modified other_func\n");	
	printf("===stack===\n");
	// xor eax, eax; ret; int3;
	int foo = 0xccc3c031;
	printf("foo: 0x%x at %p\n", foo, &foo);
	printf("Calling foo\n");
	void (*bar)() = (void (*)()) (&foo);
	printf("bar: 0x%x at %p\n", bar, &bar);
	(*bar)();
	printf("Returned from foo\n");
}
