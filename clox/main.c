#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Node Node;
struct Node {
    Node *prev;
    Node *next;
    char *string;
};

typedef struct {
    Node *start;
    Node *end;
} List;

List *createList() {
    List *list = malloc(sizeof(List));
    list->end = list->start = NULL;
    return list;
}

void insertBegin(List *list, char *string) {
    if (list == NULL)
        return;
    
    size_t size = strlen(string);
    Node *node = malloc(sizeof(Node));
    node->string = malloc(sizeof(char) * (size + 1));
    node->string[size] = '\0';
    strcpy(node->string, string);
    node->next = list->start;
    node->prev = NULL;

    if (node->next != NULL)
        node->next->prev = node;

    list->start = node;
    if (list->end == NULL)
        list->end = node;
}

void insertEnd(List *list, char *string) {
    if (list == NULL)
        return;
    
    size_t size = strlen(string);
    Node *node = malloc(sizeof(Node));
    node->string = malloc(sizeof(char) * (size + 1));
    node->string[size] = '\0';
    strcpy(node->string, string);
    node->next = NULL;
    node->prev = list->end;

    if (node->prev != NULL)
        node->prev->next = node;

    list->end = node;
    if (list->start == NULL)
        list->start = node;
}

void printList(const List *list) {
    if (list == NULL)
        return;
    
    printf("=====================\n");
    Node *it = list->start;
    while (it != NULL)
    {
        printf("%s\n", it->string);
        it = it->next;
    }
}

void destroyList(List **list) {
    List *actualList = *list;
    if (actualList == NULL)
        return;

    Node *it = actualList->start;
    while (it != NULL)
    {
        Node *next = it->next;
        free(it->string);
        free(it);
        it = next;
    }
    free(actualList);

    *list = NULL;
}

int main(int argc, char **argv) {
    printf("Hello, world!\n");

    List *list = createList();
    insertBegin(list, "World");
    insertBegin(list, "Hello");
    printList(list);
    insertBegin(list, "Hello");
    insertEnd(list, "!");
    printList(list);
    destroyList(&list);

    return 0;
}
