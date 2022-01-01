#!/bin/bash
actors="$1"
split_on_commas() {
  local IFS=,
  local WORD_LIST=($1)
  for word in "${WORD_LIST[@]}"; do
    echo "$word"
  done
}
i=0
split_on_commas "$actors" | while read item; do
  # Custom logic goes here
  IMAGE_HOST_PKEY=$(wash claims inspect ./actors/${item}/target/wasm32-unknown-unknown/release/${item}_s.wasm -o json)

  echo IMAGE_HOST_PKEY: ${IMAGE_HOST_PKEY}
  i=$((i+1))
done