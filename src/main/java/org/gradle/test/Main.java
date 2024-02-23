package org.gradle.test;

public class Main {
    public static void main(String[] args) {
        System.load(System.getProperty("user.dir") + "/build/cargo/debug/libfile_events.dylib");

        String output = FileEvents.hello("Dezső");
        System.out.println(output);
    }
}
