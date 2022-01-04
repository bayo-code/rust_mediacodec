# Documenting things to do

## TODOs
- [X] Write proper examples
- [ ] Implement returning actual buffers for raw video samples returned by the codec. So far, the decoder can only return hardware buffer samples.
- [X] Add a script to automate running adb logcat with the correct PID
- [X] Write Documentation
- [ ] Implement MediaMuxer bindings (Since there's already MediaExtractor, it's only fitting that I implement MediaMuxer too)
- [ ] Implement ImageReader. This one will be interesting, because there are plenty of use cases for why I might want to really implement it. However, I don't need it at the moment so I'll see where this goes