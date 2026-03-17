M4_FILES := $(wildcard *.m4)
DOT_FILES := $(M4_FILES:.m4=.dot)

.PHONY: all test clean

all: test

test: $(DOT_FILES)
	@for f in $(M4_FILES:.m4=); do \
		echo "Testing $$f..."; \
		diff $$f.dot $$f.dot.exp || exit 1; \
	done
	@echo "All tests passed!"

%.dot: %.m4 ../bpmn.m4
	m4 -I.. $< > $@

clean:
	rm -f *.dot
