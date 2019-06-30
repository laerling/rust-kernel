PHONY+=default
default:
	# blank target for now (because no underlying OS)
	cargo build --target thumbv7em-none-eabihf

.PHONY: $(PHONY)
