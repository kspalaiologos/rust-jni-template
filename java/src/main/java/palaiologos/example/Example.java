package palaiologos.example;

class Example {
    static {
        try {
            load(resourceName());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }
    
    public static native void printHi(void);

    public static void main(String[] args) {
        printHi();
    }
}

