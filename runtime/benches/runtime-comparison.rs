mod build;
mod manual_implementations;

use std::time::Duration;

use build::yes_microspeech_runtime_debug;
use criterion::{criterion_group, criterion_main, Criterion};
use rune_runtime::{DefaultEnvironment, Runtime};
use crate::{
    build::{
        GESTURE_DEBUG, GESTURE_RELEASE, GESTURE_RUNEFILE, MICROSPEECH_DEBUG,
        MICROSPEECH_RELEASE, MICROSPEECH_RUNEFILE, SINE_DEBUG, SINE_RELEASE,
        SINE_RUNEFILE, compile, ring_gesture_runtime_release,
        ring_gesture_runtime_debug, slope_gesture_runtime_release,
        slope_gesture_runtime_debug, wing_gesture_runtime_release,
        wing_gesture_runtime_debug, yes_microspeech_runtime_release,
    },
    manual_implementations::{ManualGesture, ManualMicrospeech, ManualSine},
};

fn main() {
    env_logger::init();

    criterion_main!(benches);
    main();
}

criterion_group!(
    benches,
    startup_times,
    execute_sine_times,
    execute_gesture_times,
    execute_microspeech_times,
    compile_times,
);

pub fn compile_times(c: &mut Criterion) {
    let mut group = c.benchmark_group("compile");

    group
        .sample_size(10)
        .measurement_time(Duration::from_secs(60));

    group
        .bench_function("sine-debug", |b| {
            b.iter(|| compile("sine", SINE_RUNEFILE, false))
        })
        .bench_function("sine-release", |b| {
            b.iter(|| compile("sine", SINE_RUNEFILE, true))
        })
        .bench_function("gesture-debug", |b| {
            b.iter(|| compile("gesture", GESTURE_RUNEFILE, false))
        })
        .bench_function("gesture-release", |b| {
            b.iter(|| compile("gesture", GESTURE_RUNEFILE, true))
        })
        .bench_function("microspeech-debug", |b| {
            b.iter(|| compile("microspeech", MICROSPEECH_RUNEFILE, false))
        })
        .bench_function("microspeech-release", |b| {
            b.iter(|| compile("microspeech", MICROSPEECH_RUNEFILE, true))
        });

    group.finish();
}

pub fn startup_times(c: &mut Criterion) {
    let mut group = c.benchmark_group("startup");

    group
        .bench_function("sine-debug", |b| {
            b.iter_with_setup(DefaultEnvironment::default, |env| {
                Runtime::load(&SINE_DEBUG, env).unwrap()
            })
        })
        .bench_function("sine-release", |b| {
            b.iter_with_setup(DefaultEnvironment::default, |env| {
                Runtime::load(&SINE_RELEASE, env).unwrap()
            })
        })
        .bench_function("gesture-debug", |b| {
            b.iter_with_setup(DefaultEnvironment::default, |env| {
                Runtime::load(&GESTURE_DEBUG, env).unwrap()
            })
        })
        .bench_function("gesture-release", |b| {
            b.iter_with_setup(DefaultEnvironment::default, |env| {
                Runtime::load(&GESTURE_RELEASE, env.clone()).unwrap()
            })
        })
        .bench_function("microspeech-debug", |b| {
            b.iter_with_setup(DefaultEnvironment::default, |env| {
                Runtime::load(&MICROSPEECH_DEBUG, env.clone()).unwrap()
            })
        })
        .bench_function("microspeech-release", |b| {
            b.iter_with_setup(DefaultEnvironment::default, |env| {
                Runtime::load(&MICROSPEECH_RELEASE, env.clone()).unwrap()
            })
        });

    group.finish();
}

pub fn execute_sine_times(c: &mut Criterion) {
    let mut group = c.benchmark_group("execute-sine");

    group
        .bench_function("debug", |b| {
            b.iter_with_setup(
                || {
                    Runtime::load(&SINE_DEBUG, DefaultEnvironment::default())
                        .unwrap()
                },
                |mut runtime| runtime.call(),
            )
        })
        .bench_function("release", |b| {
            b.iter_with_setup(
                || {
                    Runtime::load(&SINE_RELEASE, DefaultEnvironment::default())
                        .unwrap()
                },
                |mut runtime| runtime.call(),
            )
        })
        .bench_function("handwritten", |b| {
            b.iter_with_setup(ManualSine::load, |mut m| m.call())
        });

    group.finish();
}

pub fn execute_gesture_times(c: &mut Criterion) {
    let mut group = c.benchmark_group("execute-gesture");

    group
        .bench_function("wing-debug", |b| {
            b.iter_with_setup(wing_gesture_runtime_debug, |mut runtime| {
                runtime.call()
            })
        })
        .bench_function("wing-release", |b| {
            b.iter_with_setup(wing_gesture_runtime_release, |mut runtime| {
                runtime.call()
            })
        })
        .bench_function("wing-manual", |b| {
            b.iter_with_setup(ManualGesture::wing, |mut runtime| runtime.call())
        })
        .bench_function("ring-debug", |b| {
            b.iter_with_setup(ring_gesture_runtime_debug, |mut runtime| {
                runtime.call()
            })
        })
        .bench_function("ring-release", |b| {
            b.iter_with_setup(ring_gesture_runtime_release, |mut runtime| {
                runtime.call()
            })
        })
        .bench_function("ring-manual", |b| {
            b.iter_with_setup(ManualGesture::ring, |mut runtime| runtime.call())
        })
        .bench_function("slope-debug", |b| {
            b.iter_with_setup(slope_gesture_runtime_debug, |mut runtime| {
                runtime.call()
            })
        })
        .bench_function("slope-release", |b| {
            b.iter_with_setup(slope_gesture_runtime_release, |mut runtime| {
                runtime.call()
            })
        })
        .bench_function("slope-manual", |b| {
            b.iter_with_setup(ManualGesture::slope, |mut runtime| {
                runtime.call()
            })
        });

    group.finish();
}

pub fn execute_microspeech_times(c: &mut Criterion) {
    let mut group = c.benchmark_group("execute-microspeech");

    group
        .bench_function("debug", |b| {
            b.iter_with_setup(yes_microspeech_runtime_debug, |mut runtime| {
                runtime.call()
            })
        })
        .bench_function("release", |b| {
            b.iter_with_setup(yes_microspeech_runtime_release, |mut runtime| {
                runtime.call()
            })
        })
        .bench_function("manual", |b| {
            b.iter_with_setup(ManualMicrospeech::yes, |mut runtime| {
                runtime.call()
            })
        });

    group.finish();
}
