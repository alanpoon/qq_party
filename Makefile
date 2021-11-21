.PHONY: build
build:
	HOSTID=$(shell $(WASH) ctl get hosts -o json | jq -r ".hosts[0].id")
	echo $HOSTID
	wash ctl apply $HOSTID ops2/manifest.yaml