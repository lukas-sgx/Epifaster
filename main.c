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

int add_arg(char **cmd, char *arg)
{
    char *new_buffer = realloc(*cmd, sizeof(char) *
        (strlen(*cmd) + strlen(arg) + 2));

    if (new_buffer == NULL)
        return 84;
    *cmd = new_buffer;
    strcat(*cmd, " ");
    strcat(*cmd, arg);
    return 0;
}

int add_plugin(char **cmd)
{
    char *new_buffer = realloc(*cmd, sizeof(char) * (strlen(*cmd)
            + strlen(" -fplugin=" EPICLANG_PLUGIN_PATH) + 1));

    if (new_buffer == NULL) {
        free(*cmd);
        return 84;
    }
    *cmd = new_buffer;
    strcat(*cmd, " -fplugin=" EPICLANG_PLUGIN_PATH);
    system(*cmd);
    free(*cmd);
    return 0;
}

int main(int argc, char **argv)
{
    char *cmd = malloc(sizeof(char) * (strlen(EPICLANG_COMPILER) + 1));

    if (cmd == NULL)
        return 84;
    strcpy(cmd, EPICLANG_COMPILER);
    for (int i = 1; i < argc; i++) {
        if (add_arg(&cmd, argv[i]) != 0) {
            free(cmd);
            return 84;
        }
    }
    if (add_plugin(&cmd) != 0)
        return 84;
    return 0;
}
