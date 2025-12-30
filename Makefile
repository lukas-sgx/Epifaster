##
## EPITECH PROJECT, 2025
## ~/epitech/free-project
## File description:
## Makefile
##

CC = clang
CFLAGS = -Wall -Wextra -Werror -Iinclude

SRC = main.c
OBJ = $(SRC:.c=.o)

all: install

install: $(OBJ)
	@$(CC) $(CFLAGS) $(OBJ) -o epiclang
	@echo "epiclang banana checker installed!"

clean:
	@rm -f $(OBJ)
	@echo "Cleaned!"

fclean: clean
	@rm -f epiclang
	@echo "Fclean done!"

re: clean all
