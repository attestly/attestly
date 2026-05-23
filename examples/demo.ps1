# Attestly end-to-end demo — 90-second screencast script (PowerShell).
#
# 1. Initialize a fresh ledger + Ed25519 keys
# 2. Append 50 synthetic credit-scoring decisions
# 3. Publish a Signed Tree Head (Merkle root + Ed25519 signature)
# 4. Export a regulator-facing evidence bundle for decision #23 — verify PASSES
# 5. Operator tampers with the database (bypassing the append-only trigger)
# 6. Re-export and re-verify — verify FAILS, exposing the tamper
#
# Usage: pwsh examples/demo.ps1 [-TargetId N] [-Decisions N]

# Default target seq=24. With the deterministic synthetic data, every 3rd
# decision is 'denied'. We tamper a denied decision to approved so the
# payload bytes definitely change — narrative is "operator approves a
# denied loan after the fact".
param(
    [int]$TargetId = 24,
    [int]$Decisions = 50
)

$ErrorActionPreference = "Stop"

$ProjectRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$DemoDir = Join-Path $ProjectRoot "demo-out"
$Attestly = Join-Path $ProjectRoot "target\release\attestly.exe"

if (-not (Test-Path $Attestly)) {
    Write-Error "attestly.exe not found at $Attestly. Build first: cargo build --release"
    exit 1
}

if (Test-Path $DemoDir) { Remove-Item -Recurse -Force $DemoDir }
New-Item -ItemType Directory -Path $DemoDir | Out-Null
Push-Location $DemoDir

function Heading($s) { Write-Host "`n=== $s ===" -ForegroundColor Cyan }

try {
    Heading "1. Init: keys + ledger"
    & $Attestly init `
        --db attestly.db `
        --system-did "did:web:bank.example/ai-system/credit-scorer" `
        --org-did "did:web:bank.example" `
        --key-id "ops-2026-05" `
        --keys-dir keys

    Heading "2. Append $Decisions synthetic credit decisions"
    for ($i = 1; $i -le $Decisions; $i++) {
        $decision = if ($i % 3 -eq 0) { "denied" } else { "approved" }
        $score = "{0:F2}" -f ((($i * 37) % 100) / 100)
        $data = "{`"decision`":`"$decision`",`"score`":$score,`"applicant`":`"applicant-$i`"}"
        & $Attestly append --db attestly.db --keys-dir keys --data $data | Out-Null
    }
    Write-Host "    $Decisions decisions appended"

    Heading "3. Publish signed checkpoint"
    & $Attestly publish-checkpoint --db attestly.db --keys-dir keys --out checkpoint.json

    Heading "4. Export evidence bundle for decision #$TargetId"
    & $Attestly export `
        --db attestly.db --keys-dir keys `
        --decision-id $TargetId `
        --checkpoint checkpoint.json `
        --out "regulator-$TargetId.zip"

    Heading "5. Verify CLEAN bundle"
    & $Attestly verify --bundle "regulator-$TargetId.zip"
    if ($LASTEXITCODE -ne 0) { throw "expected PASS but verifier exited $LASTEXITCODE" }

    Heading "6. Operator tampers: changes decision='approved' on seq=$TargetId"
    & $Attestly demo-tamper --db attestly.db --seq $TargetId --new-decision "approved"

    Heading "7. Re-export tampered evidence"
    & $Attestly export `
        --db attestly.db --keys-dir keys `
        --decision-id $TargetId `
        --checkpoint checkpoint.json `
        --out "regulator-$TargetId-tampered.zip"

    Heading "8. Verify TAMPERED bundle (expecting FAIL)"
    & $Attestly verify --bundle "regulator-$TargetId-tampered.zip"
    if ($LASTEXITCODE -eq 0) { throw "tampered bundle verified as clean — demo failed" }

    Write-Host "`n[OK] Tamper detection works end-to-end." -ForegroundColor Green
    Write-Host "     Artifacts in: $DemoDir"
}
finally {
    Pop-Location
}
