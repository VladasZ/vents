#!/bin/bash
set -eox pipefail

rustup component add clippy

cargo clippy \
  -- \
  \
  -W clippy::all \
  -W clippy::pedantic \
  \
  -A clippy::must-use-candidate \
  -A clippy::missing-errors-doc \
  -A clippy::missing-panics-doc \
  -A clippy::module-name-repetitions \
  -A clippy::return-self-not-must-use \
  \
  -D warnings
