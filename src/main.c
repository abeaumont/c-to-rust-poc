#include <stdio.h>

struct MyStruct {
	int a;
	char *b;
	struct MyStruct *c;
};

extern struct MyStruct* new_struct(int a, char* b, struct MyStruct* c);
extern void use_struct(struct MyStruct* a);
extern const char* to_str(struct MyStruct *a);

int main() {
	printf("Hi from C!\n");
	struct MyStruct *s = new_struct(3, "cfoo", NULL);
	use_struct(s);
	printf("After use_struct, check changes:\n%s\n", to_str(s));

	struct MyStruct *other = new_struct(2, "cbar", s);
	use_struct(other);

	return 0;
}
