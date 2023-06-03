##
## EPITECH PROJECT, 2023
## make
## File description:
## virtual no ads
##

NAME	=	virtual-no-ads

all	:	$(NAME)

$(NAME)	:
	cargo build
	mv target/debug/$(NAME) .
	sudo setcap cap_net_raw,cap_net_admin=eip $(NAME)

clean:
	cargo clean

fclean: clean
	rm -f $(NAME)

re: fclean all

.PHONY:    all clean fclean re
