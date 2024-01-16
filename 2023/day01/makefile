# Compiler to use
CC = cc

# Compiler flags
CFLAGS = -Wall -g

# Name of the executable
TARGET = main

# Source files
SRCS = main.c

# Object files directory
OBJDIR = obj

# Object files
OBJS = $(patsubst %.c,$(OBJDIR)/%.o,$(SRCS))

# input files
TEST = text.txt
INPUT = input.txt

# Default target
all: $(TARGET)

# Rule to link the object files into the executable
$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^

# Rule to compile the source files into object files
$(OBJDIR)/%.o: %.c
	@mkdir -p $(OBJDIR)
	$(CC) $(CFLAGS) -c $< -o $@

# Clean target
clean:
	rm -f $(TARGET)
	rm -rf $(OBJDIR)

test: $(TARGET)
	cat $(TEST) | ./$(TARGET)

run: $(TARGET)
	cat $(INPUT) | ./$(TARGET)

# Phony targets
.PHONY: all clean
