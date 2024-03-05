use std::error::Error;
use std::path::Path;

use crossbeam_channel::{bounded, Sender};
use jni::objects::{JClass, JObject, JString, JValue};
use jni::JNIEnv;
use notify::{Event, EventHandler, RecursiveMode, Watcher};

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_org_gradle_test_FileEvents_runLoop<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    queue: JObject,
) {
    if let Err(e) = process_events(&mut env, queue) {
        env.throw(e.to_string())
            .unwrap_or_else(|e| println!("out of luck {:?}", e))
    }
}

struct EventDispatcher {
    sender: Sender<String>,
}

impl EventHandler for EventDispatcher {
    fn handle_event(&mut self, event: notify::Result<Event>) {
        match event {
            Ok(e) => {
                println!("event: {:?}", e);
                e.paths
                    .into_iter()
                    .map(|path| path.to_string_lossy().into_owned())
                    .try_for_each(|path| self.sender.send(path))
                    .unwrap_or_else(|e| println!("Channel send error {:?}", e));
            }

            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn process_events(env: &mut JNIEnv<'_>, queue: JObject) -> Result<(), Box<dyn Error>> {
    let root_path: &Path = Path::new(".");
    println!(
        "Starting watcher in {}",
        root_path.canonicalize()?.display()
    );

    let (sender, receiver) = bounded(100);
    let mut watcher = notify::recommended_watcher(EventDispatcher { sender })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(root_path, RecursiveMode::Recursive)?;

    // Processing loop
    receiver
        .into_iter()
        .try_for_each(|path| put_to_queue(env, path, &queue))
        .unwrap_or_else(|e| println!("Put to jQueue failed {:?}", e));

    Ok(())
}

fn put_to_queue(
    env: &mut JNIEnv<'_>,
    path: String,
    queue: &JObject,
) -> Result<(), jni::errors::Error> {
    let jstring = to_jstring(env, path)?;
    env.call_method(
        queue,
        "put",
        "(Ljava/lang/Object;)V",
        &[JValue::Object(&jstring)],
    )?;
    Ok(())
}

fn to_jstring<'local>(
    env: &JNIEnv<'local>,
    path: String,
) -> Result<JString<'local>, jni::errors::Error> {
    env.new_string(path)
}
