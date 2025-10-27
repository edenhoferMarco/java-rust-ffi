package de.marcoedenhofer.javarustffi;

import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.SymbolLookup;
import java.lang.invoke.MethodHandle;
import java.nio.file.Path;

import de.marcoedenhofer.javarustffi.generated.bindings.functions_h;

public class Application {
    static void main() throws Throwable {
        generatedCall();
        manualCall();
    }

    /**
     * Calls the Rust function via the generated bindings.
     * For this to work, the shared library (libfunctions_api.so) must be installed systemwide,
     * e.g. in /usr/local/lib and the linker cache updated via sudo ldconfig.
     */
    private static void generatedCall() {
        System.out.println("Calling hello_from_rust via generated bindings...");
        functions_h.hello_from_rust();
        System.out.println("Returned from hello_from_rust.");
    }

    private static void manualCall() throws Throwable {
        String libPath = System.getenv("FUNCTIONS_API_LIB");
        if (libPath == null || libPath.isBlank()) {
            libPath = "./rust-api/target/debug/libfunctions_api.so";
        }

        Path path = Path.of(libPath).toAbsolutePath();
        System.out.println("Using native library: " + path);

        System.load(path.toString());
        SymbolLookup lookup = SymbolLookup.loaderLookup();

        var symbol = lookup.find("hello_from_rust");
        if (symbol.isEmpty()) {
            throw new UnsatisfiedLinkError("Symbol hello_from_rust not found in " + path);
        }

        MethodHandle mh = Linker.nativeLinker().downcallHandle(symbol.get(),
                FunctionDescriptor.ofVoid());

        System.out.println("Calling hello_from_rust via manual FFI...");
        mh.invoke();
        System.out.println("Returned from hello_from_rust.");
    }
}
