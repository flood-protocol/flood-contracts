#!/bin/sh
# Check the one byte identifier used in FloodPlainEncodedCalls is unique.

[ "$1" -le 255 ] || {
	echo 'Incorrect Usage'
	exit
}

forge inspect FloodPlainComplete methodIdentifiers | jq --arg identifier $(printf '%02x' "$1") '[ .[][0:2] ] | contains([$identifier])'
