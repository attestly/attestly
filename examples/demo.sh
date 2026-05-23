#!/usr/bin/env bash
#
# Attestly end-to-end demo — 90-second screencast script.
#
# 1. Initialize a fresh ledger + Ed25519 keys
# 2. Append 50 synthetic credit-scoring decisions
# 3. Publish a Signed Tree Head (Merkle root + Ed25519 signature)
# 4. Export a regulator-facing evidence bundle for decision #23 — verify PASSES
# 5. Operator tampers with the database (bypassing the append-only trigger)
# 6. Re-export and re-verify — verify FAILS, exposing the tamper
#
# Usage: bash examples/demo.sh [--target-id N] [--decisions N]
set -euo pipefail

# Default target seq=24. With the deterministic synthetic data below, every
# 3rd decision is 'denied' (seqs 3, 6, 9, 12, 15, 18, 21, 24, ...). We tamper
# a 'denied' decision to 'approved' so the payload bytes definitely change —
# the demo narrative is "operator approves a denied loan after the fact".
TARGET_ID=24
N_DECISIONS=50
while [[ $# -gt 0 ]]; do
  case "$1" in
    --target-id) TARGET_ID="$2"; shift 2 ;;
    --decisions) N_DECISIONS="$2"; shift 2 ;;
    -h|--help) sed -n '2,11p' "$0" | sed 's/^# \?//'; exit 0 ;;
    *) echo "unknown arg: $1" >&2; exit 2 ;;
  esac
done

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd -- "$SCRIPT_DIR/.." && pwd )"
DEMO_DIR="$PROJECT_ROOT/demo-out"
ATTESTLY="$PROJECT_ROOT/target/release/attestly"
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || -n "${WINDIR:-}" ]]; then
  ATTESTLY="${ATTESTLY}.exe"
fi

if [[ ! -x "$ATTESTLY" ]]; then
  echo "attestly binary not found at $ATTESTLY"
  echo "build it first: cargo build --release"
  exit 1
fi

rm -rf "$DEMO_DIR"
mkdir -p "$DEMO_DIR"
cd "$DEMO_DIR"

heading() { echo; echo "=== $1 ==="; }

heading "1. Init: keys + ledger"
"$ATTESTLY" init \
  --db attestly.db \
  --system-did "did:web:bank.example/ai-system/credit-scorer" \
  --org-did "did:web:bank.example" \
  --key-id "ops-2026-05" \
  --keys-dir keys

heading "2. Append $N_DECISIONS synthetic credit decisions"
for i in $(seq 1 "$N_DECISIONS"); do
  if (( i % 3 == 0 )); then decision="denied"; else decision="approved"; fi
  # Use awk for portable float math (bash on Windows lacks bc by default).
  score=$(awk -v i="$i" 'BEGIN { printf "%.2f", (i * 37 % 100) / 100 }')
  data="{\"decision\":\"$decision\",\"score\":$score,\"applicant\":\"applicant-$i\"}"
  "$ATTESTLY" append --db attestly.db --keys-dir keys --data "$data" >/dev/null
done
echo "    $N_DECISIONS decisions appended"

heading "3. Publish signed checkpoint"
"$ATTESTLY" publish-checkpoint --db attestly.db --keys-dir keys --out checkpoint.json

heading "4. Export evidence bundle for decision #$TARGET_ID"
"$ATTESTLY" export \
  --db attestly.db --keys-dir keys \
  --decision-id "$TARGET_ID" \
  --checkpoint checkpoint.json \
  --out "regulator-${TARGET_ID}.zip"

heading "5. Verify CLEAN bundle"
"$ATTESTLY" verify --bundle "regulator-${TARGET_ID}.zip"

heading "6. Operator tampers: changes decision='approved' on seq=$TARGET_ID"
"$ATTESTLY" demo-tamper \
  --db attestly.db \
  --seq "$TARGET_ID" \
  --new-decision "approved"

heading "7. Re-export tampered evidence"
"$ATTESTLY" export \
  --db attestly.db --keys-dir keys \
  --decision-id "$TARGET_ID" \
  --checkpoint checkpoint.json \
  --out "regulator-${TARGET_ID}-tampered.zip"

heading "8. Verify TAMPERED bundle (expecting FAIL)"
if "$ATTESTLY" verify --bundle "regulator-${TARGET_ID}-tampered.zip"; then
  echo
  echo "ERROR: tampered bundle verified as clean — demo failed"
  exit 1
fi

echo
echo "[OK] Tamper detection works end-to-end."
echo "     Artifacts in: $DEMO_DIR"
