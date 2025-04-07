#!/usr/bin/env bash

# Save a copy of the license:
license_file=$(mktemp)
cat > "$license_file" << 'EOF'
// PPM-CLI: A Command-Line Interface for compressing data using Arithmetic Coding + Prediction by
// Partial Matching
// Copyright (C) 2025  Yair Ziv
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
EOF

# Normalize license text into one string
license_text=$(tr -d '\r' < "$license_file")

# Get all rust files:
files=$(find src -path "src/*.rs")
files_valid=true

for file in $files; do
  # Read and normalize the file
  file_content=$(tr -d '\r' < "$file")

  # Compare
  if [[ $file_content = *"$license_text"* ]]; then
    echo "✔ $file contains the expected license"
  else
    files_valid=false
    echo "❌ $file does NOT contain the expected license"
  fi
done

if [[ $files_valid = true ]]; then
  printf "\nAll rust files contain the license! ( •_•)>⌐■-■  (⌐■_■)\n"
else
  printf "\nSome files are missing their license.\nEnsure it is placed at the beginning of all rust files\n"
fi
