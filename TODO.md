# Documenting things to do

## TODOs
- [ ] Write proper examples
- [ ] Find a proper way to test directly without creating an Android Studio Project (cargo-apk didn't work, because NdkMediaExtractor requires we create it from a Java thread!)
    * Turns out **cargo-apk** or **ndk-rs** (dunno which one) creates a new thread. We can just attach the thread and hopefully, it'll work now :)
- [ ] Implement returning actual buffers for raw video samples returned by the codec. So far, the decoder can only return hardware buffer samples.
- [X] Add a script to automate running adb logcat with the correct PID