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
AS = aarch64-linux-gnu-as
ASFLAGS =
LD = rust-lld

#==============================================================================
# PATHS AND SOURCES
#==============================================================================
SRC_DIR = src
BUILD_DIR = build
ASM_DIR = $(SRC_DIR)/asm
DOC_DIR := doc

RUST_SRC := $(shell find $(SRC_DIR) -name '*.rs')
ASM_SRC_S := $(shell find $(ASM_DIR) -name '*.s')
ASM_SRC_S_CAP := $(shell find $(ASM_DIR) -name '*.S')
ASM_OBJS := $(patsubst %,$(BUILD_DIR)/%.o,$(ASM_SRC_S)) \
            $(patsubst %,$(BUILD_DIR)/%.o,$(ASM_SRC_S_CAP))

CRATE_NAME := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].name')
RUST_OBJ = target/$(TARGET)/debug/lib$(CRATE_NAME).a
OBJS = $(ASM_OBJS) $(RUST_OBJ)
BOOTLOADER_BIN = bootloader.bin
LINKER_SCRIPT = linker.ld

#==============================================================================
# BUILD TARGETS
#==============================================================================

all: $(BOOTLOADER_BIN)

$(BUILD_DIR)/%.s.o: %.s
	@mkdir -p $(dir $@)
	$(AS) $(ASFLAGS) $< -o $@

# Assemble bootloader .S files
$(BUILD_DIR)/%.S.o: %.S
	@mkdir -p $(dir $@)
	$(AS) $(ASFLAGS) $< -o $@

# Compile the Rust bootloader (if it exists)
$(RUST_OBJ): $(RUST_SRC)
	@echo "Building Rust bootloader..."
	cargo build --target $(TARGET)

$(BOOTLOADER_BIN): $(OBJS) $(LINKER_SCRIPT)
	@echo "Linking bootloader: $@"
	$(LD) -flavor gnu -T $(LINKER_SCRIPT) --oformat=binary -o $(BOOTLOADER_BIN) $(OBJS)

doc:
	cargo doc --target $(TARGET) --no-deps --target-dir $(DOC_DIR)

doc-open:
	cargo doc --target $(TARGET) --no-deps --target-dir $(DOC_DIR) --open

#==============================================================================
# UTILITY TARGETS
#==============================================================================

clean:
	@echo "Cleaning bootloader artifacts..."
	cargo clean
	rm -rf $(DOC_DIR)
	rm -rf $(BUILD_DIR)
	rm -f $(BOOTLOADER_BIN)

.PHONY: all clean
