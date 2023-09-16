#!/bin/bash


# File with all URL to get domains name
url_file="assets/lists.txt"

# File with all the domains to block
output_file="assets/adsdomain.txt"

rm -rf "$output_file"
touch "$output_file"

while IFS= read -r url
do
    wget -O - "$url" >> "$output_file"
done < "$url_file"

cat "$output_file" | sort | uniq > "$output_file".tmp
mv "$output_file".tmp "$output_file"

echo "File updated successfully!."
