#!/usr/bin/env bash

if [ $# -ne 2 ]; then
  echo "Usage: $0 <token file> <site.toml file>";
  exit 0;
fi

TOKEN_FILE="$1"
SITE_TOML="$2"

IFS=" " ALL_TOKENS=( $(cut -d ' ' -f 1 $TOKEN_FILE | tr "\n" " ") )

echo ${ALL_TOKENS[@]}
echo ${#ALL_TOKENS[@]}

REP=""

for token_line in ${ALL_TOKENS[@]}; do
  token=$(echo $token_line | cut -d ' ' -f 1)
  REP="$REP, \"$token\""
done

echo
sed -i "s/\"TOKEN_REPLACE\"/${REP:2}/g" $SITE_TOML
