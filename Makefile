arch ?= x86_64
kernel := .build/kernel-$(arch).bin
iso := .build/os-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, .build/arch/$(arch)/%.o, $(assembly_source_files))

target ?= $(arch)-dagger_os
rust_os := target/$(target)/debug/libdagger_os.a


.PHONY: all clean run iso kernel

all: $(kernel)

clean:
	@rm -rf .build

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso) -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p .build/isofiles/boot/grub
	@cp $(kernel) .build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) .build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) .build/isofiles 2> /dev/null
	@rm -r .build/isofiles


$(kernel): $(rust_os) $(assembly_object_files) $(linker_script)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) \
        $(assembly_object_files) $(rust_os)
kernel:
	@RUST_TARGET_PATH=$(shell pwd) xargo build --target $(target)

# compile assembly files
.build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@





