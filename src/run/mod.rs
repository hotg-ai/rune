use log;
use runic_os::rune;
use runic_types::capability::*;
use runic_types::provider::*;

pub fn run(container: &str, number_of_runs: i32) {
    log::info!("Running rune: {}", container);

    //TODO get container with uuid or tag

    let rand_capability = Capability::init(CAPABILITY::Rand, |_: &CapabilityRequest| -> Vec<u8> {
        use rand::prelude::*;
        let x: f32 = random();

        return x.to_be_bytes().to_vec();
    });

    use std::collections::HashMap;

    let mut cli_provider = Provider {
        capabilities: HashMap::new(),
    };

    cli_provider.register_capability(rand_capability);

    let r = rune::Rune::init(container, cli_provider);

    //Start benchmark
    log::info!("Executing Rune executions:{} ", number_of_runs);
    for _ in 0..number_of_runs {
        r.call();
    }
}
