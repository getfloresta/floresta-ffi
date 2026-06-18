# Add project specific ProGuard rules here.
# You can control the set of applied configuration files using the
# proguardFiles setting in build.gradle.

# for JNA
-dontwarn java.awt.*
-keep class com.sun.jna.* { *; }
-keep class org.getfloresta.* { *; }
-keepclassmembers class * extends org.getfloresta.* { public *; }
-keepclassmembers class * extends com.sun.jna.* { public *; }