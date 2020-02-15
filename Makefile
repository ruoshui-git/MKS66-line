
.PHONY: run convert clean

run:
	cargo run

convert:
	cargo run
	convert img.ppm img.png

clean:
	cargo clean
	rm *.ppm *.png