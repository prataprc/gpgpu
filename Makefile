build:
	cargo check
	cargo check --bin cgm --features cgm
	cargo check --bin fonts --features fonts
	cargo check --bin vk --features vk
	cargo check --bin wgpu --features wgpu
	cargo check --example cls
	cargo check --example bezier
	cargo check --example event_loop
	cargo check --example points
	cargo check --example triangle
	cargo check --example wirecube
	cargo check --example circle
	cargo doc

run-examples:
	cargo run --example cls
	cargo run --example bezier
	#cargo run --example event_loop
	cargo run --example points
	cargo run --example triangle
	cargo run --example wirecube -- --vertices examples/wirecube/cube.txt --rotate 1,1,1
	cargo run --example circle -- --radius 200
	cargo run --example circle -- --radius 200 --fill
