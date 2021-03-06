.PHONY: run convert clean

CONV = convert

run:
	cargo run
	$(CONV) img.ppm img.png

gen:
	cargo run

convert:
	$(CONV) img.ppm img.png

clean:
	cargo clean
	rm *.ppm *.png