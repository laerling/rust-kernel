target/x86_64-blog_os/debug/bootimage-blog_os.bin: $(WILDCARD src/*.rs) $(WILDCARD src/**/*.rs)
	cargo bootimage

PHONY+=run
run: target/x86_64-blog_os/debug/bootimage-blog_os.bin
	cargo xrun

PHONY+=deps
deps:
	# cargo fails if crate already installed, so || true
	cargo install cargo-xbuild || true
	cargo install bootimage --version "^0.7.3" || true
	rustup component add rust-src
	rustup component add llvm-tools-preview

PHONY+=clean
clean:
	find target -name '*blog_os*.bin' -delete
	find src -name '*~' -delete

.PHONY: $(PHONY)
