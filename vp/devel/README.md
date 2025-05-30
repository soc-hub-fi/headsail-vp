# Renode Platform Description files (.repl)

The contents of this directory specify the Headsail virtual prototype implemented by
Renode. The initial version was generated using Kactus2 (sa.
`../kactus2-generated-*`), and adapted for use (this directory).

The VP is structured as follows:

| Path         | Purpose |
| :-           | :-      |
| headsail.repl  | Defines all headsail CPUs, memories and peripherals |

## Supported environment variables

| Variable     | Purpose |
| :-:          | :- |
| DLA_VP_QUIET | Suppresses stdout |
| DLA_VP_OUT32 | Enable 32-bit output |
