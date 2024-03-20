# Headsail BSP

Minimal BSP for testing the virtual prototype.

## Running examples

```sh
# Run on HPC
RUSTFLAGS="-C link-arg=-Tmem_hpc.x -C link-arg=-Tlink.x" cargo build --example panic -Fpanic-uart -Fhpc-rt --target riscv64imac-unknown-none-elf
renode --console -e "set bin @$(find target -name panic | grep riscv64); include @../../scripts/2_run_hpc.resc"

# Run on SysCtrl
RUSTFLAGS="-C link-arg=-Tmem_sysctrl.x -C link-arg=-Tlink.x" cargo build --example panic -Fpanic-uart -Fsysctrl-rt --target riscv32imc-unknown-none-elf
renode --console -e "set bin @$(find target -name panic | grep riscv32); include @../../scripts/2_run_sysctrl.resc"
```
