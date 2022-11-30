#include <stdio.h>
#include <sys/auxv.h>

int main(int argc, char *argv[], char *envp[]) {
	long *val = (long *) getauxval(AT_RANDOM);
	char *plat = (char *) getauxval(AT_PLATFORM);
	printf("AT_RANDOM: %ld at 0x%p\n", *val, val);
	printf("AT_PLATFORM: %s \n", plat);
}
