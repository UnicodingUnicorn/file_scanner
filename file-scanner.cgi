#!/bin/bash

saveIFS=$IFS
IFS='=&'
params=($QUERY_STRING)
IFS=$saveIFS

declare -A array
for ((i = 0; i<${#params[@]}; i+=2))
do
   array[${params[i]}]=${params[i+1]}
done

if [ ${array[hashes]} ]; then
  export FILE_SCANNER_HASHES=${array[hashes]}
  export FILE_SCANNER_DELIMITER=,
  cd /usr/app
  cargo clean --quiet --release --target x86_64-pc-windows-gnu -p file_scanner2
  cargo build --quiet --release --target x86_64-pc-windows-gnu
  echo Content-Type: application/vnd.microsoft.portable-executable 
  echo Content-Disposition: attachment\; filename=file_scanner.exe
  echo ""
  echo "$(cat target/x86_64-pc-windows-gnu/release/file_scanner2.exe)"
else
  echo Status: 400 Bad Request
  echo ""
  echo 400 Bad Request: hashes query field is missing
fi

exit 0
