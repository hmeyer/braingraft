void putc(char c);

void print(const char *str, int len) {
    for (int i = 0; i < len; ++i) {
        putc(str[i]);
    }
}

int main() {
    const char msg[] = "Hello, world!\n";
    print(msg, sizeof(msg) - 1);
    return 0;
}
