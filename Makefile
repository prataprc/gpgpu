build:
	cargo check
	cargo check --bin vk --features vk
	cargo check --bin wgpu --features wgpu
	# cargo check --bin fonts --features fonts
	cargo check --example event_loop
	# cargo check --example render
	cargo doc
