build:
	cargo check
	cargo check --bin cgm --features cgm
	cargo check --bin fonts --features fonts
	cargo check --bin vk --features vk
	cargo check --bin wgpu --features wgpu
	cargo check --example cls
	cargo check --example event_loop
	cargo check --example points
	cargo check --example triangle
	cargo check --example wirecube
	cargo doc
