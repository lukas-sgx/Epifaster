/*
** EPITECH PROJECT, 2025
** ~/epitech/free-project
** File description:
** main
*/
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include "epiclang.h"

int main(int argc, char **argv)
{
    char *cmd = malloc(sizeof(char) * (strlen(EPICLANG_COMPILER) + 1));
    char *new_buffer = NULL;

    if (cmd == NULL)
        return 84;
    strcpy(cmd, EPICLANG_COMPILER);
    for (int i = 1; i < argc; i++) {
        new_buffer = realloc(cmd, sizeof(char) * (strlen(cmd) + strlen(argv[i]) + 2));
        if (new_buffer == NULL) {
            free(cmd);
            return 84;
        }
        cmd = new_buffer;
        strcat(cmd, " ");
        strcat(cmd, argv[i]);
    }
    new_buffer = realloc(cmd, sizeof(char) * (strlen(cmd) + strlen(" -fplugin=" EPICLANG_PLUGIN_PATH) + 1));
    if (new_buffer == NULL) {
        free(cmd);
        return 84;
    }
    cmd = new_buffer;
    strcat(cmd, " -fplugin=" EPICLANG_PLUGIN_PATH);
    system(cmd);
    free(cmd);
    return 0;
}
