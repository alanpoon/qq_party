i=0
for d in ./actors/*;do [[ -d "$d" ]] &&
  item="$(basename $d)"
  echo "$item"
  if [ -f "./actors/${item}/build/${item}_s.wasm" ]; then
    IMAGE_HOST_PKEY=$(wash claims inspect ./actors/${item}/build/${item}_s.wasm -o json | jq ".module" | tr -d "\"")
    if [ "${i}" -eq "0" ];
    then
      sed "s/!$item/$IMAGE_HOST_PKEY/g" ops2/manifest-template.yaml > ops2/manifest-temp.yaml
    else
      sed "s/!$item/$IMAGE_HOST_PKEY/g" ops2/manifest-temp.yaml2 > ops2/manifest-temp.yaml
    fi
    cp ops2/manifest-temp.yaml ops2/manifest-temp.yaml2
    echo Item: ${item}
    echo IMAGE_HOST_PKEY: ${IMAGE_HOST_PKEY}
    i=$((i+1))
  fi
done
rm ops2/manifest-temp.yaml2
mv ops2/manifest-temp.yaml ops2/manifest.yaml