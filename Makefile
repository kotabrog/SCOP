NAME := scop_display

release:
	@cd scop && cargo build --release
	@cp scop/target/release/scop ./$(NAME)

debug:
	@cd scop && cargo build
	@cp scop/target/debug/scop ./$(NAME)

$(NAME): release

all: release

clean:
	@cd SCOP && cargo clean

fclean: clean
	@rm -f ./$(NAME)

re: fclean all

.PHONY: all clean fclean re release debug
