use std::path::Path;

use crossbeam_channel::{bounded, Receiver, Sender};
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue};
use notify::{Event, RecursiveMode, Watcher};

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_org_gradle_test_FileEvents_runLoop<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    queue: JObject,
) {
    println!("Starting watcher in {}", Path::new(".").canonicalize().unwrap().display());

    let (sender, receiver): (Sender<String>, Receiver<String>) = bounded(100);

    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
        match res {
            Ok(event) => {
                println!("event: {:?}", event);
                for path in event.paths {
                    if let Ok(path_str) = path.into_os_string().into_string() {
                        if sender.send(path_str).is_err() {
                            println!("Channel send error");
                            break;
                        }
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("."), RecursiveMode::Recursive).unwrap();

    // Processing loop
    for path_str in receiver {
        // Convert to Java string and call the queue's put method
        let jstr_path = env.new_string(&path_str)
            .expect("Couldn't create java string!");
        env.call_method(&queue, "put", "(Ljava/lang/Object;)V", &[JValue::Object(jstr_path.as_ref())])
            .expect("Could not call put method on BlockingQueue");
    }
}
