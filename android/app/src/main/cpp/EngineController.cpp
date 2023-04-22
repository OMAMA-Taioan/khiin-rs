//
// Created by aiong on 3/20/2023.
//


#include <jni.h>

#include <android/asset_manager.h>
#include <android/asset_manager_jni.h>

#include "engine/Engine.h"
#include "proto/proto/command.pb.h"

using namespace khiin::engine;

extern "C"
JNIEXPORT jlong JNICALL
Java_be_chiahpa_khiin_EngineManager_load(
        JNIEnv *env,
        jobject thiz,
        jstring dbFileName
) {
    jboolean is_copy;
    const char *c_str = env->GetStringUTFChars(dbFileName, &is_copy);
    std::string db_file_name{c_str};
    auto engine = Engine::Create(db_file_name);
    return (jlong) engine.release();
}

extern "C"
JNIEXPORT jbyteArray JNICALL
Java_be_chiahpa_khiin_EngineManager_sendCommand(
        JNIEnv *env,
        jobject thiz,
        jlong engine_ptr,
        jbyteArray req_bytes
) {
    khiin::proto::Command cmd;

    auto *req_buf = env->GetByteArrayElements(req_bytes, nullptr);
    auto req_size = env->GetArrayLength(req_bytes);
    try {
        cmd.mutable_request()->ParseFromArray(
                reinterpret_cast<unsigned char *>(req_buf), req_size);
    } catch (...) { /* TODO: Handle parsing error? */ }

    env->ReleaseByteArrayElements(req_bytes, req_buf, JNI_ABORT);

    auto *engine = reinterpret_cast<Engine *>(engine_ptr);
    engine->SendCommand(&cmd);

    auto res_bytes = cmd.response().SerializeAsString();
    auto *res_buf = res_bytes.data();
    auto res_size = (int) res_bytes.size();

    auto ret = env->NewByteArray(res_size);
    env->SetByteArrayRegion(ret, 0, res_size, (jbyte *) res_buf);

    return ret;
}

extern "C"
JNIEXPORT void JNICALL
Java_be_chiahpa_khiin_EngineManager_shutdown(
        JNIEnv *env,
        jobject thiz,
        jlong engine_ptr
) {
    delete reinterpret_cast<Engine *>(engine_ptr);
}
