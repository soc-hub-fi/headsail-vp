*** Variables ***
${MACH_SCRIPT}                  ${CURDIR}/../resc/1_hpc.resc
${BUILD_SCRIPT}                 ${CURDIR}/../../examples/hpc/test-memory-maps/build.sh
${CPU}                          sysbus.cpu_hpc0
${UART}                         sysbus.apb_uart_0
${BIN}                          ${CURDIR}/../../examples/hpc/test-memory-maps/target/riscv64imac-unknown-none-elf/test-memory-maps

*** Settings ***
Suite Setup     Setup
Suite Teardown  Teardown
Test Teardown   Test Teardown
Resource        ${RENODEKEYWORDS}

*** Keywords ***
Create Machine
    Execute Script              ${MACH_SCRIPT}

*** Test Cases ***
APB UART0 is read
    Create Machine
    Create Terminal Tester      ${UART}

    ${DEVICE}                   apb_uart0
    Execute Script              ${BUILD_SCRIPT}
    Execute Command             set bin @${BIN}
    Execute Command             sysbus LoadELF $bin false true ${CPU}
    Start Emulation

    Wait For Line On Uart       [PASS]
