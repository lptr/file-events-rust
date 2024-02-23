package org.gradle.test;

import java.util.concurrent.BlockingQueue;

public class FileEvents {
    public static native void runLoop(BlockingQueue<String> queue);
}
