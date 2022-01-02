# Documenting things to do

## TODOs
- [ ] Write proper examples
- [ ] Find a proper way to test directly without creating an Android Studio Project (cargo-apk didn't work, because NdkMediaExtractor requires we create it from a Java thread!)
- [ ] Implement returning actual buffers for raw video samples returned by the codec. So far, the decoder can only return hardware buffer samples.