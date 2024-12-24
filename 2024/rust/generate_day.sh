#!/usr/bin/env bash

main() {
  
  # read user input for which day
  read -p "Enter Day Number (day #): " day

  # check whether the day already exists
  dirname=$(printf 'day_%02d' "$day")
  if [[ -d "$dirname" ]]; then
    echo "Day $day already exists"
    exit 1
  fi

  # read input file to tmpfile for error handling
  local tmpfile; tmpfile=$(mktemp /tmp/input_body.XXXXXX)
  # read http_response and http status_code
  status_code=$(curl -s --cookie session=$(cat .envrc) -w "%{http_code}" -o >(cat >"$tmpfile") "https://adventofcode.com/2024/day/$day/input") || code="$?"
  # if error then output the error but DONT write to disk
  echo "status=$status_code"
  if [[ $status_code -ne 200 ]]; then
    echo "Error: Abnormal response when getting input. Outputting http response..."
    cat "$tmpfile"
    rm "$tmpfile"
    exit 2
  fi
  # if no error, write input file to day_0N directory
  
  # generate the directory for the new day
  cargo generate --path ./daily_template --name $dirname

  echo "Writing input file to $dirname/input.txt"
  cat "$tmpfile" > "$dirname/input.txt"
  rm "$tmpfile"
}

main
exit 0

