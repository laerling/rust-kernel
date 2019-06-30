target/x86_64-blog_os/debug/bootimage-blog_os.bin:
	cargo bootimage

PHONY+=run
run: target/x86_64-blog_os/debug/bootimage-blog_os.bin
	qemu-system-x86_64 -drive format=raw,file=$+
	
.PHONY: $(PHONY)
