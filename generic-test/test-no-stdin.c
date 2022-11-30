#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// uncomment to test stack, heap or code segment for W^X
// THIS MIGHT CRASH THE TEST PROGRAM!
// #define STACK
// #define HEAP
// #define CODE

void __attribute__ ((noinline)) func() {
	return;
}

void print_stack(long *marker_addr) {
	printf("Stack frame:\n");
    for (int i = -10; i <= 10; i++) {
        printf("%p: \t 0x%lx\n", (marker_addr+i), *(marker_addr+i));
    }
}

void __attribute__ ((noinline)) print_canary() {
	long marker = 0x666;
	printf("Canary: 0x%lx @ %p\n", *((&marker)+0x1), (&marker)+0x1);
}

void __attribute__ ((noinline)) call_target() {
	printf("Called target!\n");
}

void __attribute__ ((noinline)) get_input() {
	printf("Please input a string with more than 8 byte to overflow the stack buffer:\n");
	long marker = 0x42424242;
	char input[8];
	input[0] = 'C';
	print_stack(&marker);
	// change limit to overflow buffer
	for (int i=0; i<8; i++) {
		input[i] = 'A';
	}
	print_stack(&marker);
    printf("%s", input);
}

int main(int argc, char* argv[]) {
	printf("---------------------------------\n");
	printf("Welcome to the generic unikernel security test script!\n");
	printf("----- ASLR -----\n");
	//Test for ASLR, execute multiple times and compare the addresses
	static int static_var = 42;
	int stack_var = 23;
	char* heap_var = malloc(8);
	*heap_var = 'A';
	printf("static_var: %d @ %p\n", static_var, &static_var);
	printf("stack_var: %d @ %p\n", stack_var, &stack_var);
	printf("heap_var: 0x%x @ %p\n", *heap_var, heap_var);
	printf("func: @ %p\n", &func);


	printf("----- W^X ------\n");
	printf("!!! Executing this tests might crash the unikernel due to page faults!!!\n");
	printf("Please uncomment the according #define if you really want to execute W^X tests!\n");

	// Test stack segment executable
	#ifdef STACK
	int code = 0xccc3d3ff;
	void (*ptr)() = (void (*)())&code;
	printf("Attempting to execute code on stack...\n");
	ptr();
	printf("Done!\n");
	#endif

	// Test heap segment executable
	#ifdef HEAP
	int *code = malloc(8);
    *code = 0xccc3d3ff;           
    printf("Function code @ %p: %p\n", code, *code);
    printf("Attempting to execute code on heap...\n");               
    ((void (*)())code)();                                            
    printf("Done!\n"); 
	#endif

	// Test code segment writable
	#ifdef CODE
	printf("func: %p @ %p\n", *func, &func);
	printf("Attempting to overwrite function pointer content...\n");
	*((long*)&func) = 0xccccccc3;
	printf("Written!\n");
	func();
	printf("Executed manipulated function!\n");
	#endif

	printf("--- function calls for stack canary check -----\n");
	func();
	call_target();
	print_canary();
	get_input();
}

