#==============================================================================
# AArch64 Bootloader Build System
#==============================================================================
# This Makefile builds the bootloader independently.
# It can be used standalone or called from the kernel's Makefile.
#
# Main targets:
#   make all          - Build bootloader
#   make clean        - Clean build artifacts
#   make doc          - Generate documentation
#==============================================================================

#==============================================================================
# TOOLCHAIN CONFIGURATION
#==============================================================================
TARGET = aarch64-unknown-none
CC = aarch64-linux-gnu-gcc-14
CFLAGS = -c -I$(INCLUDE_DIR) -x assembler-with-cpp
AS = aarch64-linux-gnu-as
ASFLAGS = -I$(INCLUDE_DIR)
CPP = aarch64-linux-gnu-cpp-14
CPPFLAGS = -I$(INCLUDE_DIR)
OBJCOPY = aarch64-linux-gnu-objcopy
LD = aarch64-linux-gnu-ld
QEMU = qemu-system-aarch64
QEMU_FLAGS = -nographic -machine virt,gic-version=3,virtualization=on -cpu cortex-a57 -kernel $(BOOTLOADER_BIN) -s -S

#==============================================================================
# PATHS AND SOURCES
#==============================================================================
SRC_DIR := src
INCLUDE_DIR := include
BUILD_DIR := build
ASM_DIR := $(SRC_DIR)/asm
DOC_DIR := doc

RUST_SRC := $(shell find $(SRC_DIR) -name '*.rs')
ASM_SRC_S := $(shell find $(ASM_DIR) -name '*.s')
ASM_SRC_S_CAP := $(shell find $(ASM_DIR) -name '*.S')
ASM_OBJS := $(patsubst %,$(BUILD_DIR)/%.o,$(ASM_SRC_S)) \
            $(patsubst %,$(BUILD_DIR)/%.o,$(ASM_SRC_S_CAP))

CRATE_NAME := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].name')
RUST_OBJ := target/$(TARGET)/debug/lib$(CRATE_NAME).a
OBJS := $(ASM_OBJS) $(RUST_OBJ)
BOOTLOADER_ELF := bootloader.elf
BOOTLOADER_BIN := bootloader.bin
LINKER_SCRIPT := linker.lds

#==============================================================================
# BUILD TARGETS
#==============================================================================

all: $(BOOTLOADER_BIN)

run: all
	$(QEMU) $(QEMU_FLAGS)

$(LINKER_SCRIPT).tmp: $(LINKER_SCRIPT) $(INCLUDE_DIR)/asm/boot.h
	$(CPP) $(CPPFLAGS) -P -C $< -o $@

$(BUILD_DIR)/%.s.o: %.s
	@mkdir -p $(dir $@)
	$(AS) $(ASFLAGS) $< -o $@

$(BUILD_DIR)/%.S.o: %.S
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) $< -o $@

$(RUST_OBJ): $(RUST_SRC)
	@echo "Building Rust bootloader..."
	cargo build --target $(TARGET)

$(BOOTLOADER_ELF): $(OBJS) $(LINKER_SCRIPT).tmp
	@echo "Linking bootloader ELF: $@"
	$(LD) -T $(LINKER_SCRIPT).tmp -o $(BOOTLOADER_ELF) $(OBJS)

$(BOOTLOADER_BIN): $(BOOTLOADER_ELF)
	@echo "Extracting raw binary: $@"
	$(OBJCOPY) -O binary $(BOOTLOADER_ELF) $(BOOTLOADER_BIN)

doc:
	cargo doc --target $(TARGET) --no-deps --target-dir $(DOC_DIR)

doc-open:
	cargo doc --target $(TARGET) --no-deps --target-dir $(DOC_DIR) --open

clean:
	@echo "Cleaning bootloader artifacts..."
	cargo clean
	rm -rf $(LINKER_SCRIPT).tmp
	rm -rf $(DOC_DIR)
	rm -rf $(BUILD_DIR)
	rm -f $(BOOTLOADER_ELF)
	rm -f $(BOOTLOADER_BIN)

.PHONY: all clean
