IMAGE := target/target_config/debug/bootimage-os-dev.bin

build:
	cargo bootimage

run: build
	qemu-system-x86_64 \
	-drive format=raw,file=$(IMAGE)

clean:
	cargo clean
