# IPN Sim
A Delay Tolerant Networking (DTN) simulator designed with the InterPlanetary Network (IPN) in mind. Implemented in Rust.

## Features
- A static [web app](https://goodingc.github.io/ipn_sim/web-app/)
- Dynamic planetary bodies with signal occlusion
- A (*small but expanding*) selection of DTN routers:
  - [Epidemic](http://issg.cs.duke.edu/epidemic/epidemic.pdf)
    - Vanilla
    - Acknowledged
  - [Spray and Wait](https://dl.acm.org/doi/10.1145/1080139.1080143)
- Reporting capabilities
- An *early stage* static [router zoo](https://goodingc.github.io/ipn_sim/router-zoo/)
- A router profiler

## Roadmap
- General cleanup
- Documentation
- Scenario config file parser
- Multithreading support in executable
  - Possible incremental reporting model
- Multithreading support for wasm builds
- More routers
- Workaround for constant message parsing