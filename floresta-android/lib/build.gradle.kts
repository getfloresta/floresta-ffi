import com.vanniktech.maven.publish.AndroidSingleVariantLibrary
import com.vanniktech.maven.publish.JavadocJar
import com.vanniktech.maven.publish.SourcesJar
import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.dokka")
    id("com.vanniktech.maven.publish")
}

group = "org.getfloresta"
version = "0.1.0-SNAPSHOT"

android {
    namespace = group.toString()
    compileSdk = 36

    defaultConfig {
        minSdk = 24
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
            proguardFiles(file("proguard-android-optimize.txt"), file("proguard-rules.pro"))
        }
    }
}

tasks.withType<KotlinCompile> {
    compilerOptions {
        jvmTarget.set(JvmTarget.JVM_17)
    }
}

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(17))
    }
}

dependencies {
    implementation("net.java.dev.jna:jna:5.14.0@aar")
    implementation("androidx.core:core-ktx:1.7.0")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.1")
    api("org.slf4j:slf4j-api:1.7.30")

    androidTestImplementation("com.github.tony19:logback-android:2.0.0")
    androidTestImplementation("androidx.test.ext:junit:1.3.0")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.4.0")
    androidTestImplementation("org.jetbrains.kotlin:kotlin-test:1.6.10")
    androidTestImplementation("org.jetbrains.kotlin:kotlin-test-junit:1.6.10")
}

mavenPublishing {
    coordinates(
        groupId = group.toString(),
        artifactId = "floresta-android",
        version = version.toString()
    )

    pom {
        name.set("floresta-android")
        description.set("Floresta Bitcoin Utreexo full node language bindings for Android.")
        url.set("https://github.com/getfloresta/floresta-ffi")
        inceptionYear.set("2024")
        licenses {
            license {
                name.set("MIT")
                url.set("https://github.com/getfloresta/floresta-ffi/blob/master/LICENSE")
            }
        }
        developers {
            developer {
                id.set("getfloresta")
                name.set("Floresta Developers")
            }
        }
        scm {
            url.set("https://github.com/getfloresta/floresta-ffi/")
            connection.set("scm:git:github.com/getfloresta/floresta-ffi.git")
            developerConnection.set("scm:git:ssh://github.com/getfloresta/floresta-ffi.git")
        }
    }

    configure(
        AndroidSingleVariantLibrary(
            javadocJar = JavadocJar.Dokka("dokkaGeneratePublicationHtml"),
            sourcesJar = SourcesJar.Sources(),
            variant = "release",
        )
    )

    publishToMavenCentral()
    if (!providers.gradleProperty("skipSigning").isPresent) {
        signAllPublications()
    }
}

dokka {
    moduleName.set("floresta-android")
    moduleVersion.set(version.toString())
    pluginsConfiguration.html {
        footerMessage.set("(c) Floresta Developers")
    }
}