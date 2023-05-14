package palaiologos.example;

import java.lang.ref.Cleaner;

class CleanerSingleton {
    static final Cleaner CLEANER = Cleaner.create();
}
