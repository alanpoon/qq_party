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
  IMAGE_HOST_PKEY=$(wash claims inspect ./actors/${item}/target/wasm32-unknown-unknown/release/${item}_s.wasm -o json | jq ".module" | tr -d "\"")
  if [ "${i}" -eq "0" ];
  then
    sed "s/!$item/$IMAGE_HOST_PKEY/g" ops/manifest-template.yaml > ops/manifest-temp.yaml
  else
    sed "s/!$item/$IMAGE_HOST_PKEY/g" ops/manifest-temp.yaml2 > ops/manifest-temp.yaml
  fi
  cp ops/manifest-temp.yaml ops/manifest-temp.yaml2
  echo Item: ${item}
  echo IMAGE_HOST_PKEY: ${IMAGE_HOST_PKEY}
  i=$((i+1))
done
rm ops/manifest-temp.yaml2
mv ops/manifest-temp.yaml ops/manifest.yaml