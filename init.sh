#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Use directly from the year folder"
    echo "Usage: ../init.sh <day-number>"
    exit 1
fi

# DESTINATION="$1"
DAY="aoc${1}"
FOLDER="./aoc${1}"
TEMPLATE="../template"

if [ ! -d "$TEMPLATE" ]; then
    echo "Error: template folder not found"
    exit 1
fi

cp -r "$TEMPLATE" "${FOLDER}"
echo "Copied template to ${FOLDER}"

touch ${FOLDER}/src/test.txt
touch ${FOLDER}/src/input.txt
echo "Created empty test.txt and input.txt in ${FOLDER}/src"

sed -i "s/template/${DAY}/" "${FOLDER}/Cargo.toml"

## WIP
# # 1. Get the current members array
# MEMBERS=$(toml get Cargo.toml "workspace.members")

# echo "Members $MEMBERS"

# NEW=$(echo "$MEMBERS" | jq --arg "$DAY" '. + ["$day"]' )

# echo "Members $NEW"

# # 3. Set the new array back to the TOML file
# toml set Cargo.toml "workplace.members" "$NEW" > temp && mv temp Cargo.toml
