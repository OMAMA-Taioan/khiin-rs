import org.jetbrains.kotlin.gradle.plugin.extraProperties

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("com.google.protobuf")
//    id("org.mozilla.rust-android-gradle.rust-android")
    id("io.github.MatrixDev.android-rust")
}

android {
    compileSdk = 33
    namespace = "be.chiahpa.khiin"
    ndkVersion = "25.2.9519653"

    defaultConfig {
        applicationId = "be.chiahpa.khiin"
        minSdk = 26
        targetSdk = 33
        versionCode = 1
        versionName = "1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"

        vectorDrawables {
            useSupportLibrary = true
        }
    }

    sourceSets {
        getByName("main") {
            java.srcDir("src/main/proto")
            java.srcDir("src/main/rust")
        }
    }

    buildTypes {
        getByName("release") {
            isMinifyEnabled = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }

    compileOptions {
        sourceCompatibility(JavaVersion.VERSION_1_8)
        targetCompatibility(JavaVersion.VERSION_1_8)
    }

    kotlinOptions {
        jvmTarget = "1.8"
    }

    buildFeatures {
        compose = true
    }

    composeOptions {
        kotlinCompilerExtensionVersion = "1.4.3"
    }

    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
}

androidRust {
    module("library") {
        path = file("../rust")
    }
    minimumSupportedRustVersion = "1.61.1"
}

protobuf {
    protoc {
        artifact = "com.google.protobuf:protoc:3.21.12"
    }

    generateProtoTasks {
        all().forEach {
            it.builtins {
                create("java") {
                    option("lite")
                }
                create("kotlin") {
                    option("lite")
                }
            }
        }
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.9.0")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("com.google.android.material:material:1.8.0")

    implementation("androidx.lifecycle:lifecycle-runtime-compose:2.6.0")
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.6.0")
    implementation("androidx.lifecycle:lifecycle-viewmodel-compose:2.6.0")

    implementation("androidx.activity:activity-compose:1.6.1")
    implementation(platform("androidx.compose:compose-bom:2022.10.00"))
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.ui:ui-graphics")
    implementation("androidx.compose.ui:ui-tooling-preview")
    implementation("androidx.compose.material3:material3")

    implementation("androidx.datastore:datastore-preferences:1.0.0")

    protobuf(files("../../protos/src"))
    implementation("com.google.protobuf:protobuf-javalite:3.21.12")
    implementation("com.google.protobuf:protobuf-kotlin-lite:3.21.12")

    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
    androidTestImplementation(platform("androidx.compose:compose-bom:2022.10.00"))
    androidTestImplementation("androidx.compose.ui:ui-test-junit4")
    debugImplementation("androidx.compose.ui:ui-tooling")
    debugImplementation("androidx.compose.ui:ui-test-manifest")
}

//cargo {
//    module = "./src/main/rust"
//    libname = "khiin_droid"
//    targets = listOf("arm", "arm64", "x86", "x86_64")
//    targetDirectory = "../../target"
//}

//project.afterEvaluate {
//    val jniTargetDirs = mutableListOf<File>();
//    tasks.withType(com.nishtahir.CargoBuildTask::class.java).forEach {
//        jniTargetDirs.add(File("$buildDir/rustJniLibs", it.toolchain!!.folder))
//    }
//
//    tasks.matching { it.name.matches("merge.*JniLibFolders".toRegex()) }
//        .forEach { jniTargetDirs.forEach { dir -> it.inputs.dir(dir) } }
//}

//gradle.taskGraph.whenReady {
//    val javaPreCompileDebugTask =
//        project.tasks.findByName("javaPreCompileDebug")
//    if (javaPreCompileDebugTask != null) {
//        val cargoBuildTask = project.tasks.findByName("cargoBuild")
//        if (cargoBuildTask != null) {
//            javaPreCompileDebugTask.dependsOn(cargoBuildTask)
//        }
//    }
//}
