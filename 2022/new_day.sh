#!/bin/sh

day_num=$1

# # new day directory name
day_dir="day_${day_num}"

# make dir and init it with rust
cargo init $day_dir

# Replace main with the template file
rm "${day_dir}/src/main.rs"
cp ./template.rs "${day_dir}/src/main.rs"
