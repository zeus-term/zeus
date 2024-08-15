alias dz := dev-zeus
alias dt := dev-term

dev-zeus:
  @cd zeus && cargo run

dev-term:
  @cd hermes && cargo build && printf "\n"
  echo "Build successful hermes"
  echo ""
  ./hermes/target/debug/hermes

build:
  @cd hermes && cargo build --release
  @cd zeus && cargo build --release
