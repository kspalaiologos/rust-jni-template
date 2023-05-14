package palaiologos.example;

class Example {
    static {
        try {
            load(resourceName());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }
    
    public static native void printHi();

    public static void main(String[] args) {
        printHi();
        
        ExampleManagedObject obj = new ExampleManagedObject("some message", 42);
        System.out.println(obj.frobnicate());
    }
}

