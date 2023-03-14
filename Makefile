NAME := scop_display

release:
	@cd SCOP && cargo build --release
	@cp SCOP/target/release/scop ./$(NAME)

debug:
	@cd SCOP && cargo build
	@cp SCOP/target/debug/scop ./$(NAME)

$(NAME): release

all: release

clean:
	@cd SCOP && cargo clean

fclean: clean
	@rm -f ./$(NAME)

re: fclean all

.PHONY: all clean fclean re release debug
