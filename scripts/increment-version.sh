#!/bin/bash

### Increments the part of the string
## $1: version itself
## $2: part to update: major.minor.patch

increment_version() {
  local part=$(case $2 in
    "major") echo "0" ;;
    "minor") echo "1" ;;
    "patch") echo "2" ;;
    *) echo $2 ;;
  esac)

  local delimiter=.
  local array=($(echo "$1" | tr $delimiter '\n'))
  array[$part]=$((array[$part]+1))
  if [ $part -lt 2 ]; then array[2]=0; fi
  if [ $part -lt 1 ]; then array[1]=0; fi
  echo $(local IFS=$delimiter ; echo "${array[*]}")
}

