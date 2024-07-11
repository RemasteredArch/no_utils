#! /usr/bin/env bash

# SPDX-License-Identifier: AGPL-3.0-or-later
#
# Copyright © 2024 RemasteredArch
#
# This file is part of onebrc. onebrc is a part of no_utils.
#
# no_utils is free software: you can redistribute it and/or modify it under the terms of the GNU
# Affero General Public License as published by the Free Software Foundation, either version 3 of
# the License, or (at your option) any later version.
#
# no_utils is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
# even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
# Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License along with no_utils. If
# not, see <https://www.gnu.org/licenses/>.

# CreateMeasurements.sh: a wrapper for CreateMeasurements.java

set -euo pipefail # Quit upon any error or attempt to access unset variables


#### CONFIG ####


script_name="create_measurements.sh"
script_source="$0"
script_source_dir=$(dirname "$script_source")
default_length="1_000"


#### HELPER UTILS ####


text() {
  local color_name="$1"
  local color=""

  case $color_name in
    bold )
      color="\e[1m"
      ;;
    dim | faint )
      color="\e[2m"
      ;;
    red )
      color="\e[31m"
      ;;
    reset | normal | * )
      color="\e[0m"
      ;;
  esac

  echo -e "$color"
}

error() {
  echo "$(text red)$*$(text reset)" >&2 # Prints to stderr
}

fatal_error() {
  error "$@"

  exit 1
}

opt() {
  local 
  echo "$(text faint)$1$(text reset) | $(text faint)$2$(text reset)"
}


#### FUNCTIONS ####


help() {
  cat << EOF
$(text bold)$script_name$(text reset): a wrapper for CreateMeasurements.java

$(text bold)Usage:$(text reset) $script_name [options] {measurement file size}

Where options options are the following:
  $(opt -h --help)  Displays this help message

Where measurement file size is the number of lines in the output, specified as either:
  - A number (1000)
  - A number, separated by any non-digit (regex: [^0-9]) (1_000, 1,000, 1.000, etc.)
  - A word (ten, hundred, thousand, million, billion, or trillion)
EOF
}

get_length() {
  case "$1" in
    ten )
      set -- "10"
      ;;
    hundred )
      set -- "100"
      ;;
    thousand )
      set -- "1_000"
      ;;
    million )
      set -- "1_000_000"
      ;;
    billion )
      set -- "1_000_000_000"
      ;;
    trillion )
      set -- "1_000_000_000_000"
      ;;
  esac

  local lines="${1//[^0-9]/}"

  # If the above emptied the string, reset it
  [ -z "$lines" ] && lines="${default_length//[^0-9]/}"

  echo "$lines"
}

parse_args() {
  set -- "${@:-$default_length}" # Set a default value

  while true; do
    case "$1" in
      -h | --help )
        help
        exit 0
        ;;
      -- )
        shift
        break
        ;;
      * )
        break
        ;;
    esac
  done

  get_length "$1"
}

get_generator() {
  local generator_path="$script_source_dir/dev/morling/onebrc/CreateMeasurements.java"

  [ -f "$generator_path" ] || fatal_error "Can't find generator at $generator_path!"

  echo "$generator_path"
}


#### LOGIC ####


generator=$(get_generator)
lines=$(parse_args "$@")
output="measurements_$lines.txt"

java --source 21 "$generator" "$lines" "$output"

echo "($output)"
