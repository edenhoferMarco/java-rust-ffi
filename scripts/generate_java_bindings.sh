# start the script from project root
./scripts/jextract-25/bin/jextract \
  --output java-client/src/main/java \
  --target-package de.marcoedenhofer.javarustffi.generated.bindings \
  --library functions_api \
  rust-api/include/functions.h