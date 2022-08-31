#include <stdio.h>

#define KIR(n) while(n) { \
printf("text %s", n); \
}

typedef struct {
    char first;
    char second;
    char third;
    short fourth;
} sample_t;
void print_size(sample_t* var) {
    printf("Size: %lu bytes\n", sizeof(*var));
}
void print_bytes(sample_t* var) {
    unsigned char* ptr = (unsigned char*)var;
    for (int i = 0; i < sizeof(*var); i++, ptr++) {
        printf("%c ", (char )*ptr);
    }
    printf("\n");
}
int main(int argc, char** argv) {
    sample_t var = {'A', 'B', 'C', 765 };
    print_size(&var);
    print_bytes(&var);
    KIR("Hello");
    return 0;
}