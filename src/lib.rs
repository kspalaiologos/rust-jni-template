// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString, JObject};

// Raw objects.
use jni::sys::{jint, jlong, jstring};

#[no_mangle]
pub extern "system" fn Java_palaiologos_example_Example_printHi(_env: JNIEnv, _class: JClass) {
    println!("Hello from JNI!")
}

struct ExampleManagedObject {
    name: String,
    value: i32,
}

impl ToString for ExampleManagedObject {
    fn to_string(&self) -> String {
        format!("ExampleManagedObject {{ name: {}, value: {} }}", self.name, self.value)
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_example_ExampleManagedObject_alloc(
    env: JNIEnv,
    _class: JClass,
    arg1: JString,
    arg2: jint
) -> jlong {
    let obj: ExampleManagedObject = ExampleManagedObject {
        name: env.get_string(arg1).unwrap().into(),
        value: arg2,
    };
    let ptr = Box::into_raw(Box::new(obj));
    let ptr = ptr as jlong;
    ptr
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_example_ExampleManagedObject_free(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong
) {
    let ptr = ptr as *mut ExampleManagedObject;
    unsafe { ptr.drop_in_place(); }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_example_ExampleManagedObject_frobnicate(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong
) -> jstring {
    let ptr = ptr as *mut ExampleManagedObject;
    let n = unsafe { &*ptr };
    let s = n.to_string();
    match env.new_string(s) {
        Ok(s) => s.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate string."));
            JObject::null().into_raw()
        }
    }
}

// Stub out some unwinding functions.

macro_rules! stub {
    ($name:ident) => {
        #[no_mangle]
        #[cfg(feature = "unwind-stubs")]
        pub extern "C" fn $name() -> ! { loop {} }
    };
}

stub!(_Unwind_Resume);
stub!(_Unwind_GetIP);
stub!(_Unwind_GetLanguageSpecificData);
stub!(_Unwind_GetRegionStart);
stub!(_Unwind_SetGR);
stub!(_Unwind_GetGR);
stub!(_Unwind_SetIP);
stub!(_Unwind_GetDataRelBase);
stub!(_Unwind_Backtrace);
stub!(_Unwind_GetIPInfo);
stub!(_Unwind_GetTextRelBase);

