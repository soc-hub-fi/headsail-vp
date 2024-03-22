#!/bin/sh

# Runs compiled ELF from $BIN or param $1 on HPC

set -x # echo on

BASEDIR=$(dirname "$0")
$BASEDIR/setup_renode.sh

BIN=${BIN=$1}
renode --console -e "set bin @$BIN; include @$BASEDIR/resc/2_run_hpc.resc"
