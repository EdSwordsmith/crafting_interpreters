#include "list.h"

ListResult create_list() {
    List *list = malloc(sizeof(List));
    if (list == NULL)
        return (ListResult) ERR;
    return (ListResult) OK(list);
}

NodeResult push_front(ListResult list_result, char *string) {
    if (list_result.error)
        return (NodeResult) ERR;
    List *list = list_result.ok;
    
    size_t size = strlen(string);
    Node *node = malloc(sizeof(Node));
    if (node == NULL)
        return (NodeResult) ERR;

    node->string = malloc(sizeof(char) * (size + 1));
    if (node->string == NULL)
        return (NodeResult) ERR;
    
    node->string[size] = '\0';
    strcpy(node->string, string);
    node->next = list->start;
    node->prev = NULL;

    if (node->next != NULL)
        node->next->prev = node;

    list->start = node;
    if (list->end == NULL)
        list->end = node;

    return (NodeResult) OK(node);
}

NodeResult push_back(ListResult list_result, char *string) {
    if (list_result.error)
        return (NodeResult) ERR;
    List *list = list_result.ok;
    
    size_t size = strlen(string);
    Node *node = malloc(sizeof(Node));
    if (node == NULL)
        return (NodeResult) ERR;
    
    node->string = malloc(sizeof(char) * (size + 1));
    if (node->string == NULL)
        return (NodeResult) ERR;
    
    node->string[size] = '\0';
    strcpy(node->string, string);
    node->next = NULL;
    node->prev = list->end;

    if (node->prev != NULL)
        node->prev->next = node;

    list->end = node;
    if (list->start == NULL)
        list->start = node;

    return (NodeResult) OK(node);
}

void print_list(const ListResult list_result) {
    if (list_result.error)
        return;
    List *list = list_result.ok;

    printf("=====================\n");
    Node *it = list->start;
    while (it != NULL)
    {
        printf("%s\n", it->string);
        it = it->next;
    }
}

void destroy_list(ListResult list_result) {
    if (list_result.error)
        return;
    List *list = list_result.ok;

    Node *it = list->start;
    while (it != NULL)
    {
        Node *next = it->next;
        free(it->string);
        free(it);
        it = next;
    }
    free(list);
}

NodeResult find_string(ListResult list_result, const char *string) {
    if (list_result.error)
        return (NodeResult) ERR;
    List *list = list_result.ok;
    
    Node *it = list->start;
    while (it != NULL)
    {
        if (strcmp(it->string, string) == 0)
            return (NodeResult) OK(it);
        it = it->next;
    }

    return (NodeResult) ERR;
}

void delete_node(ListResult list_result, NodeResult node) {
    if (node.error || list_result.error)
        return;
    Node *actual_node = node.ok;

    if (actual_node->prev != NULL)
        actual_node->prev->next = actual_node->next;

    if (actual_node->next != NULL)
        actual_node->next->prev = actual_node->prev;

    if (actual_node == list_result.ok->start)
        list_result.ok->start = actual_node->next;
    
    if (actual_node == list_result.ok->end)
        list_result.ok->end = actual_node->prev;
    
    free(actual_node->string);
    free(actual_node);
}
