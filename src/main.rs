use surge_rs::SurgeSynthesizer;
use textplots::{Chart, Plot, Shape};

fn main() {
    let mut synth = SurgeSynthesizer::new(48000.0);

    synth.play_note(0, 60, 127, 0, 0, 0);

    let mut l_samples: Vec<_> = Vec::new();
    let mut r_samples: Vec<_> = Vec::new();
    for _ in 0..100 {
        synth.process();
        l_samples.extend_from_slice(&synth.get_samples()[0]);
        r_samples.extend_from_slice(&synth.get_samples()[1]);
    }

    let l_data: Vec<_> = l_samples.iter().enumerate().map(|(i, &s)| (i as f32, s)).collect();
    let r_data: Vec<_> = r_samples.iter().enumerate().map(|(i, &s)| (i as f32, s)).collect();

    println!("lightning surge from the clouds:");
    Chart::new(200, 80, 0.0, l_data.len() as f32)
        .lineplot(&Shape::Lines(&l_data))
        .display();
    println!("this one's from the right:");
    Chart::new(200, 30, 0.0, r_data.len() as f32)   // more cache-friendly to use l_data?
        .lineplot(&Shape::Lines(&r_data))
        .display();
}
