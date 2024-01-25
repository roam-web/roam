set windows-shell := ["cmd.exe", "/c"]

dev:
	cargo run

release:
	cargo run --release

doc:
	cargo doc --open
