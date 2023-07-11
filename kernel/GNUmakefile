# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

PACKAGE_TEST_EXCLUDES = kernel-paging
PACKAGE_TEST_EXCLUDES += kernel-paging-impl
PACKAGE_TEST_EXCLUDES += kernel-serial
PACKAGE_TEST_EXCLUDES += kernel-serial-impl
PACKAGE_TEST_EXCLUDES += kernel-boot
PACKAGE_TEST_EXCLUDES += kernel-boot-impl
PACKAGE_TEST_EXCLUDES += kernel-cpu
PACKAGE_TEST_EXCLUDES += kernel-cpu-impl
PACKAGE_TEST_EXCLUDES += kernel-shutdown
PACKAGE_TEST_EXCLUDES += kernel-shutdown-impl
PACKAGE_TEST_EXCLUDES += kernel-test
PACKAGE_TEST_EXCLUDES += kernel-log
PACKAGE_TEST_EXCLUDES += kernel-boot-interface
PACKAGE_TEST_EXCLUDES += teensy-std

# Default target.
.PHONY: all
all:
	cargo build --target x86_64-unknown-none
	cp target/x86_64-unknown-none/debug/odysseos kernel.elf

.PHONY: run
run: all
	cd .. ; make run

.PHONY: test
test:
	cargo t --target x86_64-unknown-none --workspace $(foreach package, $(PACKAGE_TEST_EXCLUDES), --exclude $(package))

# Remove object files and the final executable.
.PHONY: clean
clean:
	cargo clean
	rm -rf kernel.elf

.PHONY: distclean
distclean: clean
