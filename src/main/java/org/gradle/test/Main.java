package org.gradle.test;

public class Main {
    public static void main(String[] args) {
        System.loadLibrary("file-events");

        String output = FileEvents.hello("World");
        System.out.println(output);
    }
}
