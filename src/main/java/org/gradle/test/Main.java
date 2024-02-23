package org.gradle.test;

import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;

public class Main {
    public static void main(String[] args) throws IOException {
        Path libFile = Files.createTempFile("libfile_events", ".dylib");
        try (InputStream resourceStream = Main.class.getClassLoader().getResourceAsStream("libfile_events.dylib")) {
            if (resourceStream == null) {
                throw new IllegalArgumentException("Cannot find library");
            }
            Files.copy(resourceStream, libFile, StandardCopyOption.REPLACE_EXISTING);
        }
        try {
            System.load(libFile.toAbsolutePath().toString());

            runWithNativeLibraryLoaded();
        } finally {
            Files.delete(libFile);
        }
    }

    private static void runWithNativeLibraryLoaded() {
        String output = FileEvents.hello("Dezső");
        System.out.println(output);
    }
}
