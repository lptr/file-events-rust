package org.gradle.test;

import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.LinkedBlockingQueue;

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
        BlockingQueue<String> paths = new LinkedBlockingQueue<>();
        Thread watcherThread = new Thread(() -> {
            FileEvents.runLoop(paths);
            try {
                paths.put("");
            } catch (InterruptedException ignored) {
                // ignore
            }
        }, "watcher");
        watcherThread.start();
        while (true) {
            String path;
            try {
                path = paths.take();
            } catch (InterruptedException ignored) {
                System.out.println("Interrupted");
                break;
            }
            if (path.isBlank()) {
                break;
            }
            System.out.println(" - File event from RUST: " + path);
        }
        System.out.println("Finished on Java side");
    }
}
