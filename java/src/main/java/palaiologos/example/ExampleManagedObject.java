package palaiologos.example;

import java.lang.ref.Cleaner;
import static palaiologos.example.NativeLibrary.*;

import java.io.IOException;

class ExampleManagedObject {
    static {
        try {
            load(resourceName());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }
    
    // Free the pointer when object is out of scope.
    static class CleanerRunnable implements Runnable {
        private final long pointer;

        CleanerRunnable(long pointer) {
            this.pointer = pointer;
        }

        @Override
        public void run() {
            System.out.println("Freeing ExampleManagedObject");
            ExampleManagedObject.free(pointer);
        }
    }

    private final long ptr;
    private final Cleaner.Cleanable cleanable;

    private static native long alloc(String s, int n);
    private static native void free(long ptr);
    private static native String frobnicate(long ptr);

    public ExampleManagedObject(String s, int n) {
        System.out.println("Allocating ExampleManagedObject");
        this.ptr = alloc(s, n);
        cleanable = CleanerSingleton.CLEANER.register(this, new CleanerRunnable(ptr));
    }

    public String frobnicate() { return frobnicate(ptr); }
}

