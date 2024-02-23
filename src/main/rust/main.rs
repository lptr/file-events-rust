use std::io;
use std::path::Path;

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue};
use notify::{Event, RecursiveMode, Watcher};

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_org_gradle_test_FileEvents_runLoop<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    queue: JObject,
) {
    println!("Starting watcher in {}", Path::new(".").canonicalize().unwrap().display());

    // Create a global reference to the queue object to use it inside the closure
    let queue_global_ref = env.new_global_ref(queue)
        .expect("Couldn't create global ref for queue");
    let jvm = env.get_java_vm().expect("Couldn't get JVM");

    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
        match res {
            Ok(event) => {
                println!("event: {:?}", event);
                let mut env = jvm.attach_current_thread().expect("Failed to attach current thread");
                for path_buf in event.paths {
                    if let Ok(path_str) = path_buf.into_os_string().into_string() {
                        let jstr_path = env.new_string(path_str)
                            .expect("Couldn't create java string!");
                        env.call_method(&queue_global_ref, "put", "(Ljava/lang/Object;)V", &[JValue::Object(jstr_path.as_ref())])
                            .expect("Could not call put method on BlockingQueue");
                    }
                }
                unsafe {
                    jvm.detach_current_thread();
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("."), RecursiveMode::Recursive).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("Finished watcher in {}", Path::new(".").canonicalize().unwrap().display());
}
