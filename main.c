/*
** EPITECH PROJECT, 2025
** ~/epitech/free-project
** File description:
** main
*/
#include <string.h>
#include <stdlib.h>
#include "epiclang.h"

int main(int argc, char **argv)
{
    char *cmd = malloc(5 * sizeof(char));

    strcpy(cmd, EPICLANG_COMPILER);
    for (int i = 1; i < argc; i++) {
        strcat(cmd, " ");
        strcat(cmd, argv[i]);
    }
    strcat(cmd, " -fplugin=" EPICLANG_PLUGIN_PATH);
    system(cmd);
    free(cmd);
    return 0;
}
