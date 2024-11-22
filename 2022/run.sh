
# Check the exit status
if cargo test --bin $1; then
  echo "Tests passed"
  cargo run --bin $1 -- $2
else
  echo "Command failed"
fi
