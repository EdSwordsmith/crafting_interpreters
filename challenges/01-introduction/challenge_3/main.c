#include "list.h"

int main(int argc, char **argv) {
    printf("Hello, world!\n");

    ListResult list = create_list();
    push_front(list, "Hello");
    push_back(list, "World");
    push_back(list, "!");
    print_list(list);
    delete_node(list, find_string(list, "!"));
    print_list(list);
    destroy_list(list);

    return 0;
}
