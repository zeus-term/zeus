projects := "client master common"
alias dz := dev-zeus
alias dt := dev-term

default:
  just --list

dev-zeus:
  @cd master && cargo run

dev-term:
  @cd client && cargo run

fmt:
  #!/bin/bash
  for project in {{projects}}; do
    cd $project
    cargo fmt
    cd ..
  done

rust_release_file_size:
  #!/bin/bash
  for project in {{projects}}; do
    size=$(cd $project && ls -alih target/release/$project | awk '{print $6}')
    echo "Size of $project := $size"
  done

clean:
  #!/bin/bash
  for project in {{projects}}; do
    cd $project
    cargo clean
  done


_build_projects:
  @echo "Building Hermes..."
  @cd client && cargo build --release
  @echo "Hermes Built Successfully"
  @echo "Building Zeus..."
  @cd master && cargo build --release
  @echo "Zeus Built Successfully..."

# To build all the components of zeus
build: _build_projects && rust_release_file_size 

fmt-check:
  #!/bin/bash
  for project in {{projects}} ; do
    cd $project
    cargo fmt --all -- --check
    cd ..
  done

clippy-check-ci:
  #!/bin/bash
  for project in {{projects}} ; do
    cd $project
    cargo clippy
    cd ..
  done

clippy-check-sonar:
  #!/bin/bash
  for project in {{projects}} ; do
    cd $project
    cargo clippy --message-format=json > clippy-report.json
    cd ..
  done

test:
  #!/bin/bash
  for project in {{projects}} ; do
    cd $project
    cargo test
    cd ..
  done

