# Abstract
This is an example project how to use native libraries from Rust (via C ABI) with 
the new FFI from Java 22+

# Components
## java-client
This is the Java client which will use the functions from the rust-api
## rust-api
This is the Rust lib which contains a set of functions for clients to use.
## scripts
Here are some bash scripts located which download and execute jextract to generate 
java code from the rust-api header file.

# Development
## Build native image
This project is set up to create a native image using GraalVM's native-image tool. 
To build the native image, execute following command: `mvn -Pnative package -DskipTests`
## Execute native image
After building the native image, you can run the Java client by navigating to `target` and
executing `./java-client`
> Note: the dynamic lib from the rust-api (libfunctions_api.so) needs to be locally installed.
> 
> You can do this by copying the file to `/usr/local/lib` and updating the linker via
> `sudo ldconfig`.

# Documentation
## DeepWiki
This repository is indexed at DeepWiki. Check it out at https://deepwiki.com/edenhoferMarco/java-rust-ffi
