plugins {
    `java-library`
    application
}

application {
    mainClass = "org.gradle.test.Main"
}

@CacheableTask
abstract class CarguBuild : DefaultTask() {
    @get:InputFiles
    @get:PathSensitive(PathSensitivity.NONE)
    abstract val sources: ConfigurableFileCollection

    @get:OutputDirectory
    abstract val destinationDirectory: DirectoryProperty

    @Inject
    abstract fun getExecOperations(): ExecOperations

    @TaskAction
    fun taskAction() {
        getExecOperations().exec {
            commandLine(
                "cargo", "build",
                "--config", "build.target-dir = \"" + destinationDirectory.get().asFile.absolutePath + "\"",
            )
        }
    }
}

val buildRustLib by tasks.registering(CarguBuild::class) {
    // This is just an assumption about what files we are going to consume
    sources.from(project.layout.files("Cargo.toml", "Cargo.lock", "src/main/rust"))
    destinationDirectory = project.layout.buildDirectory.dir("rust")
}

tasks.named<ProcessResources>("processResources") {
    from(buildRustLib.flatMap { it.destinationDirectory.file("debug/libfile_events.dylib") })
}
