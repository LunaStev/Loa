VERSION := $(shell grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
TARGET_DIR := ./target
BINARY_NAME := loa

TARGETS := \
    x86_64-unknown-linux-gnu \
    x86_64-unknown-linux-musl \
    aarch64-unknown-linux-gnu \
    x86_64-pc-windows-gnu

install:
	rustup target add $(TARGETS)

build:
	@for target in $(TARGETS); do \
		cargo build --target $$target --release; \
	done

package:
	@for target in $(TARGETS); do \
		target_dir="$(TARGET_DIR)/$$target/release"; \
		output_name="$(BINARY_NAME)"; \
		formatted_target=$$(echo $$target | sed 's/-unknown//'); \
		if echo $$target | grep -q "windows"; then \
			cp "$$target_dir/$$output_name.exe" .; \
			zip $(BINARY_NAME)-v$(VERSION)-$$formatted_target.zip $$output_name.exe; \
			rm $$output_name.exe; \
		else \
			tar -czvf $(BINARY_NAME)-v$(VERSION)-$$formatted_target.tar.gz -C "$$target_dir" $$output_name; \
		fi; \
	done

release: build package

clean:
	rm -rf $(TARGET_DIR) *.zip *.tar.gz