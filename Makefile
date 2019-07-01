target/x86_64-blog_os/debug/bootimage-blog_os.bin: src/*.rs
	cargo bootimage

PHONY+=run
run: target/x86_64-blog_os/debug/bootimage-blog_os.bin
	qemu-system-x86_64 -drive format=raw,file=$+

PHONY+=deps
deps:
	# cargo fails if crate already installed, so || true
	cargo install cargo-xbuild || true
	cargo install bootimage --version "^0.7.3" || true
	rustup component add rust-src
	rustup component add llvm-tools-preview

.PHONY: $(PHONY)
