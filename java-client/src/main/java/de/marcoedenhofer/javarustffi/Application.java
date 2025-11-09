package de.marcoedenhofer.javarustffi;

import de.marcoedenhofer.javarustffi.generated.bindings.functions_h;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;

public class Application {
    static void main() {
        callWithGeneratedBindings();
    }

    /**
     * Calls the Rust function via the generated bindings.
     * For this to work, the shared library (libfunctions_api.so) must be installed systemwide,
     * e.g. in /usr/local/lib and the linker cache updated via sudo ldconfig.
     */
    private static void callWithGeneratedBindings() {
        IO.println("Calling hello_from_rust via generated bindings...");
        functions_h.hello_from_rust();
        IO.println("Returned from hello_from_rust.");

        int[] inputs = new int[] {10, 5, 2};
        var inputsAsArray = MemorySegment.ofArray(inputs);
        MemorySegment nativeAddressOfInputArray = Arena.ofAuto().allocate(inputsAsArray.byteSize());
        // populate the native memory segment with the input data
        nativeAddressOfInputArray.copyFrom(inputsAsArray);

        var handle = functions_h.get_calculation_handle(functions_h.Add(), nativeAddressOfInputArray, inputs.length);
        IO.println("Handle: " + handle);
        int result = functions_h.perform_calculation(handle);
        IO.println("Result from rust perform_calculation: " + result);
    }
}
