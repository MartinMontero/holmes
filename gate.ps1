#Requires -Version 5.1
<#
.SYNOPSIS
    gate.ps1 -- the definition-of-done gate for the Holmes workspace.

.DESCRIPTION
    Runs every proof line from RECIPE.md, in dependency order, printing
    PASS/FAIL per subsystem and ending with "X/Y subsystems PASS".
    Exits 0 only if every subsystem passes.

    Where a proof maps to an existing check (cargo test, acdl2-scan,
    recipe-scan), this gate CALLS it -- nothing is re-implemented here.

    -SelfTest: plants a failing proof (holmes-smoke's expected test count
    is set to 45 -- holmes-guard's count, impossible for holmes-smoke's 2),
    confirms the FAIL fires and the exit code is non-zero, restores the
    real expectation (2), reruns clean, and prints both transcripts as
    evidence. Exits 0 only if the plant fired AND the restored run passes.

    Platform: Windows PowerShell. No bash, no POSIX, no && chaining.
    Environment precondition: loopback 127.0.0.1:11434 must be free for the
    holmes-guard acdl1_proxy positive control (a local Ollama holds it).
    The preflight below warns; it never skips.
#>
param([switch]$SelfTest)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $Root

# --- tunables (the RECIPE.md proof numbers; drift here = DRIFT RULE) -----
$script:Expected = @{
    'holmes-guard' = 45
    'holmes-core'  = 71
    'holmes-wall'  = 24
    'holmes-smoke' = 2
}
$script:Acdl2PackagesExpected = 157
$script:Acdl2FilesExpected    = 68
$script:RecipeFilesExpected   = 2
$script:OsvIdsExpected        = @(
    'RUSTSEC-2024-0384', 'RUSTSEC-2024-0436',
    'RUSTSEC-2025-0012', 'RUSTSEC-2025-0134'
)

# Native-call wrapper: cargo writes progress to stderr even on success,
# which under ErrorActionPreference=Stop becomes NativeCommandError in
# Windows PowerShell 5.1. Locally relaxing EAP inside the function scope
# keeps stderr as plain captured text; the real exit code is read from
# $LASTEXITCODE immediately after the call.
$script:NativeExit = 0
function Invoke-Cargo {
    param([string[]]$CargoArgs)
    $ErrorActionPreference = 'Continue'
    $out = & cargo @CargoArgs 2>&1 | Out-String
    $script:NativeExit = $LASTEXITCODE
    return $out
}

# --- result machinery ------------------------------------------------------
$script:Results = @()

function Add-Result {
    param([string]$Name, [bool]$Ok, [string]$Detail)
    $script:Results += [pscustomobject]@{ Name = $Name; Ok = $Ok; Detail = $Detail }
    if ($Ok) { Write-Output ("PASS {0} -- {1}" -f $Name, $Detail) }
    else     { Write-Output ("FAIL {0} -- {1}" -f $Name, $Detail) }
}

# --- individual proofs -----------------------------------------------------

function Test-CargoCrate {
    param([string]$Package)
    $out = Invoke-Cargo @('test', '--release', '--locked', '-p', $Package)
    $exit = $script:NativeExit
    $total = 0
    foreach ($m in [regex]::Matches($out, 'test result: ok\.\s+(\d+) passed')) {
        $total += [int]$m.Groups[1].Value
    }
    $want = $script:Expected[$Package]
    $detail = "cargo test --release --locked -p $Package exit=$exit; summed passed=$total (expected $want)"
    if ($out -match 'cannot bind 127\.0\.0\.1:11434') {
        $detail += '; ENVIRONMENT: 127.0.0.1:11434 is occupied (acdl1_proxy positive control cannot bind) -- free the port (e.g. stop a local Ollama) and rerun'
    }
    return @{ Ok = (($exit -eq 0) -and ($total -eq $want)); Detail = $detail }
}

function Test-Acdl2Gate {
    $neg = Invoke-Cargo @('run', '--release', '--locked', '-p', 'holmes-guard', '--bin', 'acdl2-scan', '--', '--root', '.')
    $negExit = $script:NativeExit
    $pos = Invoke-Cargo @('run', '--release', '--locked', '-p', 'holmes-guard', '--bin', 'acdl2-scan', '--', '--root', '.', '--lockfile', 'crates/holmes-guard/tests/fixtures/planted.lock')
    $posExit = $script:NativeExit
    $pkgs  = ([regex]::Match($neg, 'packages scanned:\s+(\d+)')).Groups[1].Value
    $files = ([regex]::Match($neg, 'files scanned:\s+(\d+)')).Groups[1].Value
    $ok = ($negExit -eq 0) -and ($neg -match 'verdict: CLEAN') `
        -and ($posExit -ne 0) -and ($pos -match 'verdict: FAIL') `
        -and ($pkgs -eq "$($script:Acdl2PackagesExpected)") `
        -and ($files -eq "$($script:Acdl2FilesExpected)")
    $detail = "real tree exit=$negExit verdict CLEAN (packages scanned: $pkgs, files scanned: $files); planted.lock control exit=$posExit verdict FAIL"
    return @{ Ok = $ok; Detail = $detail }
}

function Test-RecipeScan {
    $neg = Invoke-Cargo @('run', '--release', '--locked', '-p', 'holmes-guard', '--bin', 'recipe-scan', '--', '--path', 'recipes')
    $negExit = $script:NativeExit
    $pos = Invoke-Cargo @('run', '--release', '--locked', '-p', 'holmes-guard', '--bin', 'recipe-scan', '--', '--path', 'crates/holmes-guard/tests/fixtures/planted_recipe_smuggled.yaml')
    $posExit = $script:NativeExit
    $files = ([regex]::Match($neg, 'files scanned:\s+(\d+)')).Groups[1].Value
    $ok = ($negExit -eq 0) -and ($neg -match 'verdict: CLEAN') `
        -and ($files -eq "$($script:RecipeFilesExpected)") `
        -and ($posExit -ne 0) -and ($pos -match 'verdict: FAIL')
    $detail = "recipes exit=$negExit verdict CLEAN (files scanned: $files); planted_recipe_smuggled.yaml control exit=$posExit verdict FAIL"
    return @{ Ok = $ok; Detail = $detail }
}

function Test-CiGateDefinitions {
    $acdl = '.github/workflows/acdl-gate.yml'
    $sc   = '.github/workflows/supply-chain.yml'
    $ok = (Test-Path $acdl) -and (Test-Path $sc) `
        -and ((Get-Content $acdl -Raw) -match 'acdl-joint-gate') `
        -and ((Get-Content $sc -Raw) -match 'sbom-and-cve-scan')
    $detail = "$acdl contains job acdl-joint-gate; $sc contains job sbom-and-cve-scan (live required-status on main is GitHub-side, not locally provable)"
    return @{ Ok = $ok; Detail = $detail }
}

function Test-SupplyChainPosture {
    $raw = Get-Content 'osv-scanner.toml' -Raw
    $ids = @([regex]::Matches($raw, 'RUSTSEC-\d{4}-\d{4}') | ForEach-Object { $_.Value } | Sort-Object -Unique)
    $expected = @($script:OsvIdsExpected | Sort-Object)
    $ok = ($ids.Count -eq 4) -and (($ids -join ',') -eq ($expected -join ','))
    $detail = "osv-scanner.toml suppression set = $($ids.Count) IDs [$($ids -join ', ')] (expected exactly the ledgered D-13 four)"
    return @{ Ok = $ok; Detail = $detail }
}

function Test-WindowsPortabilityFixtures {
    $raw = Get-Content 'crates/holmes-guard/tests/acdl1_spawn.rs' -Raw
    $hasWin   = ($raw -match '#\[cfg\(windows\)\]') -and $raw.Contains('C:\holmes\goose.exe')
    $hasPosix = ($raw -match '#\[cfg\(not\(windows\)\)\]') -and $raw.Contains('/opt/holmes/goose')
    $detail = 'acdl1_spawn.rs carries both cfg(windows) GOOSE_PATH=C:\holmes\goose.exe and cfg(not(windows)) GOOSE_PATH=/opt/holmes/goose (hermetic; nothing spawned)'
    return @{ Ok = ($hasWin -and $hasPosix); Detail = $detail }
}

function Test-SpecCanon {
    $p = 'docs/holmes-spec-v2.md'
    $ok = (Test-Path $p) -and ((Get-Content $p -Raw).Contains('repo copy wins'))
    $detail = "$p exists and contains the sync-rule marker 'repo copy wins' (bolded in source: '**repo copy wins**')"
    return @{ Ok = $ok; Detail = $detail }
}

# --- the gate --------------------------------------------------------------

function Invoke-Gate {
    $script:Results = @()

    # Preflight (warns, never skips): cargo present; port 11434 free.
    try { $null = Get-Command cargo -ErrorAction Stop }
    catch { Add-Result 'preflight' $false 'cargo not on PATH'; return }

    $portFree = $false
    try {
        $l = New-Object System.Net.Sockets.TcpListener([System.Net.IPAddress]::Parse('127.0.0.1'), 11434)
        $l.Start(); $l.Stop(); $portFree = $true
    } catch { $portFree = $false }
    if (-not $portFree) {
        Write-Output 'PREFLIGHT WARNING: 127.0.0.1:11434 is occupied -- the holmes-guard acdl1_proxy positive control cannot bind and that subsystem will FAIL. Free the port (e.g. stop a local Ollama) and rerun for a clean gate.'
    }

    # Dependency order: guard -> core / wall -> smoke -> scanners -> config.
    $r = Test-CargoCrate 'holmes-guard'; Add-Result 'holmes-guard' $r.Ok $r.Detail
    $r = Test-CargoCrate 'holmes-core';  Add-Result 'holmes-core'  $r.Ok $r.Detail
    $r = Test-CargoCrate 'holmes-wall';  Add-Result 'holmes-wall'  $r.Ok $r.Detail
    $r = Test-CargoCrate 'holmes-smoke'; Add-Result 'holmes-smoke' $r.Ok $r.Detail
    $r = Test-Acdl2Gate;                 Add-Result 'acdl2-denylist-gate' $r.Ok $r.Detail
    $r = Test-RecipeScan;                Add-Result 'recipe-scan'  $r.Ok $r.Detail
    $r = Test-CiGateDefinitions;         Add-Result 'ci-joint-gate' $r.Ok $r.Detail
    $r = Test-SupplyChainPosture;        Add-Result 'supply-chain-posture' $r.Ok $r.Detail
    $r = Test-WindowsPortabilityFixtures; Add-Result 'windows-portability-fixtures' $r.Ok $r.Detail
    $r = Test-SpecCanon;                 Add-Result 'spec-canon'   $r.Ok $r.Detail

    $pass = @($script:Results | Where-Object { $_.Ok }).Count
    $tot  = $script:Results.Count
    Write-Output ("{0}/{1} subsystems PASS" -f $pass, $tot)
    if ($pass -eq $tot) { $script:LastExit = 0 } else { $script:LastExit = 1 }
}

$script:LastExit = 1

if (-not $SelfTest) {
    Invoke-Gate
    exit $script:LastExit
}

# --- -SelfTest: prove the gate can FAIL ------------------------------------

Write-Output '=== SELF-TEST Run A: planting a failing proof ==='
Write-Output ("plant: holmes-smoke expected-passed {0} -> 45 (that is holmes-guard's count; holmes-smoke has 2). The gate must FAIL holmes-smoke and exit non-zero." -f $script:Expected['holmes-smoke'])
$realSmoke = $script:Expected['holmes-smoke']
$script:Expected['holmes-smoke'] = 45
Invoke-Gate
$runAExit = $script:LastExit
$smokeResult = $script:Results | Where-Object { $_.Name -eq 'holmes-smoke' }
$plantFired = ($runAExit -ne 0) -and ($null -ne $smokeResult) -and (-not $smokeResult.Ok)
if ($plantFired) {
    Write-Output "SELF-TEST evidence: plant FIRED -- holmes-smoke FAILed (exit $runAExit): $($smokeResult.Detail)"
} else {
    Write-Output "SELF-TEST BROKEN: the planted failing proof did NOT fire (run A exit $runAExit). The gate cannot be trusted to fail."
}

Write-Output '=== SELF-TEST restore: holmes-smoke expected-passed back to 2 ==='
$script:Expected['holmes-smoke'] = $realSmoke

Write-Output '=== SELF-TEST Run B: restored, clean run ==='
Invoke-Gate
$runBExit = $script:LastExit
$smokeRestored = $script:Results | Where-Object { $_.Name -eq 'holmes-smoke' }
$restoredOk = ($null -ne $smokeRestored) -and $smokeRestored.Ok
Write-Output ("SELF-TEST summary: plant fired = {0}; restored proof passes = {1}; full restored-gate exit = {2}" -f $plantFired, $restoredOk, $runBExit)
Write-Output 'SELF-TEST exit code semantics: 0 = the plant fired AND the restored proof passes again. The full restored-gate exit is reported separately above and may be non-zero for unrelated environmental reasons (it never masks a FAIL: run the plain gate for the authoritative verdict).'

if ($plantFired -and $restoredOk) { exit 0 }
exit 1
