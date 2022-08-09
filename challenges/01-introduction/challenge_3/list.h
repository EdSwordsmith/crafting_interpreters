#ifndef LIST_H
#define LIST_H

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define RESULT(T)   \
    struct          \
    {               \
        T *ok;      \
        bool error; \
    }

#define OK(V) {.ok = V, .error = false}
#define ERR {.ok = NULL, .error = true}

typedef struct Node Node;
struct Node
{
    Node *prev;
    Node *next;
    char *string;
};

typedef struct
{
    Node *start;
    Node *end;
} List;

typedef RESULT(List) ListResult;
typedef RESULT(Node) NodeResult;


ListResult create_list();
NodeResult push_front(ListResult list_result, char *string);
NodeResult push_back(ListResult list_result, char *string);
void print_list(const ListResult list_result);
void destroy_list(ListResult list_result);
NodeResult find_string(ListResult list_result, const char *string);
void delete_node(ListResult list_result, NodeResult node);

#endif /* LIST_H */
