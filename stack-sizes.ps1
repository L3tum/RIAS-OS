if ((Get-Command "stack-sizes.exe" -ErrorAction SilentlyContinue) -eq $null)
{
    cargo install stack-sizes
}

if(![System.IO.Directory]::Exists("$(pwd)/target/sysroot")){
    cargo bootimage
}
$sysroot="$(pwd)/target/sysroot"
$old_env_variable = [Environment]::GetEnvironmentVariable("RUSTFLAGS")
[Environment]::SetEnvironmentVariable("RUSTFLAGS", "--sysroot $sysroot -Zemit-stack-sizes", "User")
$env:RUSTFLAGS="--sysroot $sysroot -Zemit-stack-sizes"
cargo rustc --bin rias_os -- -C link-arg=-Tkeep_stack_sizes.x -C link-arg=-N
cargo rustc --bin rias_os --release -- -C link-arg=-Tkeep_stack_sizes.x -C link-arg=-N
stack-sizes target/x86_64/debug/rias_os > stack-sizes-debug.csv
stack-sizes target/x86_64/release/rias_os > stack-sizes-release.csv

[Environment]::SetEnvironmentVariable("RUSTFLAGS", $old_env_variable, "User")

Write-Host "Press any key to continue..."
$Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")