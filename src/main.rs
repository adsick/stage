extern crate cpal;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use std::sync::mpsc::{channel, Receiver, Sender};

use stage::{signal::*, voice::Voice, wave::WaveSignal};

//next goes basically 'debloated' beep cpal's example
//I'm only interested on the essentials...

fn main() {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("no default output device");
    println!(
        "Output device: {}",
        device.name().unwrap_or("<unknown>".to_string())
    );

    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
    };
}

//note: a lot of commented dirt left from the past experimentation

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig)
where
    T: cpal::Sample,
{
    let sr = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;

    //let fb = helpers::SineFabric::new(sample_rate);

    //let mut voices: Vec<Sine> = fb.fav(&[(440.0, 1.0), (880.0, 0.5)]); //old "voices"

    let mut voices: Vec<Voice> = vec![
        //Voice::sin(55.0, sr as u32, 2.0),
        Voice::square(20.0, sr as u32, 4.0),
    ];

    // let mut signals: Vec<Box<dyn Signal>> = vec![Box::new(WaveSignal::sin(250.0, sr as u32))];

    let (sndr, rcvr) = channel::<Box<dyn FnOnce(&mut Vec<Sine>) + Send>>();

    let mut next_value = move || {
        // if let Ok(f) = rcvr.try_recv() {
        //     f(&mut voices);
        //     //dbg!(&voices);
        // }

        let mut c = 0;
        let vs: f32 = voices.iter_mut().map(|s| {c+=1; s.sample()}).sum::<f32>()/c as f32;
        //let ss: f32 = signals.iter_mut().map(|s| s.sample()).sum();
        vs // + ss
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &mut next_value)
            },
            err_fn,
        )
        .expect("unable to build stream");
    stream.play().expect("unable to play stream");

    //this is still here to show the way sync channel can be used to inject some logic directly into running threads
    //by sending closures in there.
    #[allow(unused_must_use)]
    {
        //std::thread::sleep(std::time::Duration::from_millis(2000));

        sndr.send(Box::new(|v| v.last_mut().unwrap().f = 440.0 * 1.33484));
        println!("1.25");
        //std::thread::sleep(std::time::Duration::from_millis(1000));

        sndr.send(Box::new(|v| v.last_mut().unwrap().a = 0.0));
        sndr.send(Box::new(|v| v.iter_mut().nth_back(1).unwrap().a = 0.0));
    }

    std::thread::sleep(std::time::Duration::from_millis(8000)); //secret ingridient
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
