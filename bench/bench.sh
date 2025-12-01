# Benchmarks the interpreter by evaluating a term generating a "maximally mixed" state of dimension n.
# This is the state H^n * |0>^n. We expect to see exponential growth due to the blowup in Hilbert space.
#!/usr/bin/env bash
set -euo pipefail

QLAM="../target/debug/qlam"
MAX=12

for n in $(seq 1 $MAX); do
  infile=$(mktemp)
  outfile=$(mktemp)

  zeros=$(printf "0 %.0s" $(seq 1 $n))
  hs=$(printf "H %.0s" $(seq 1 $n))
  printf "%s\n%s" "$zeros" "$hs" >"$infile"

  echo "=== length $n ==="
  echo "compiling..."
  "$QLAM" compile "$infile" >"$outfile"
  echo "quit\n" >>"$outfile"
  echo "done."
  hyperfine "sh -c '$QLAM < $outfile'"

  rm "$infile" "$outfile"
done
