#!/usr/bin/env bash

YEAR="${1}"
DAY="${2}"
DAY_NO_ZEROS="$(echo $DAY | sed 's/^0*//')"
OUTPUT="${YEAR}/inputs/day${DAY}.txt"
PUZZLE_URL="https://adventofcode.com/${YEAR}/day/${DAY_NO_ZEROS}/input"

mkdir -p "$(dirname ${OUTPUT})"
curl "${PUZZLE_URL}" -H "cookie: session=${AOC_SESSION_COOKIE}" -o "${OUTPUT}"
