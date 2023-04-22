use jni::objects::JByteArray;
use jni::objects::JClass;
use jni::objects::JString;
use jni::sys::jlong;
use jni::JNIEnv;
use khiin::Engine;
use khiin_protos::command::Command;
use khiin_protos::command::Request;
use protobuf::Message;

#[no_mangle]
pub extern "system" fn Java_be_chiahpa_khiin_EngineManager_load<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    db_filename: JString<'local>,
) -> jlong {
    android_log::init("KhiinEngine").unwrap();
    log::debug!("Trying to initialize Khiin Engine");

    let db_filename: String = env
        .get_string(&db_filename)
        .expect("Could not get string!")
        .into();

    log::debug!("Using database from file: {}", db_filename);

    let engine = Box::new(Engine::new(&db_filename));
    if engine.is_none() {
        log::debug!("Unable to initialize engine.");
        return 0;
    }

    log::debug!("Engine intialized");
    let engine = engine.unwrap();
    Box::into_raw(Box::new(engine)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_be_chiahpa_khiin_EngineManager_sendCommand<
    'local,
>(
    // #[allow(unused_mut)]
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    engine_ptr: jlong,
    request_bytes: JByteArray<'local>,
) -> JByteArray<'local> {
    let bytes = env
        .convert_byte_array(&request_bytes)
        .expect("Could not convert byte array");

    let req = Request::parse_from_bytes(&bytes)
        .expect("Could not parse Request bytes");

    let mut cmd = Command::new();
    cmd.request = Some(req).into();
    let bytes = cmd
        .write_to_bytes()
        .expect("Could not write Command to bytes");

    let engine = unsafe { &mut *(engine_ptr as *mut Engine) };
    let bytes = engine
        .send_command_bytes(&bytes)
        .expect("Could not parse response from Engine");

    let cmd = Command::parse_from_bytes(&bytes).unwrap();
    log::debug!(
        "jni::sendCommand number of candidates: {}",
        cmd.response.candidate_list.candidates.len()
    );

    let ret = env
        .byte_array_from_slice(&bytes)
        .expect("Could not create java byte array");
    ret
}
