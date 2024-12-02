#!/usr/bin/env bash

main() {
  read -p "Enter Day Number (day #): " day
  dirname=$(printf 'day_%02d' "$day")
  cargo generate --path ./daily_template --name $dirname
  curl --cookie session="$(cat .envrc)" "https://adventofcode.com/2024/day/$day/input" > "$dirname"/input.txt
}

main

