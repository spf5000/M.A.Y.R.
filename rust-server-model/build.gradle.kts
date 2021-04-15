plugins {
    java
    id("software.amazon.smithy").version("0.5.2")
}

repositories {
    mavenLocal()
    mavenCentral()
}

buildscript {
    dependencies {
        classpath("software.amazon.smithy:smithy-openapi:1.7.0")
        classpath("software.amazon.smithy:smithy-aws-traits:1.7.0")
    }
}

dependencies {
    implementation("software.amazon.smithy:smithy-aws-traits:1.7.0")
}
