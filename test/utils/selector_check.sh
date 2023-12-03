#!/bin/sh
# Check the one byte identifier used in FloodPlain EncodedCalls is unique.

[ "$1" -le 255 ] || {
	echo 'Incorrect Usage'
	exit
}

forge inspect FloodPlain methodIdentifiers | jq --arg identifier $(printf '%02x' "$1") '[ .[][0:2] ] | contains([$identifier])'
