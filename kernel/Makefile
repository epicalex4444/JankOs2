.PHONY: all run.img

all:
	+$(MAKE) -C ../

run.img:
	+$(MAKE) run.img $(filter-out $@,$(MAKECMDGOALS)) -C ../

%:
	@: