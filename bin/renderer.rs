extern crate itertools;
extern crate weresocool;
extern crate portaudio;
extern crate socool_parser;
extern crate num_rational;
use portaudio as pa;

use itertools::Itertools;
use num_rational::{
    Ratio,
};
use weresocool::{
    event::{Event, Render},
    instrument::{
        oscillator::Oscillator,
        stereo_waveform::StereoWaveform
    },
    operations::{Apply, GetOperations},
    settings::get_default_app_settings,
    portaudio_setup::output::setup_portaudio_output,
};
use socool_parser::ast::{Op, Op::*};

fn rational_play() {
    let d = Ratio::new(1, 7);
    let e = Ratio::new(3, 2);

    println!("Hello New Renderer {}", d + e);
}


type NormOp = Op;
type Sequences = Vec<Op>;
type NormEv = Vec<Vec<Event>>;
type VecWav = Vec<StereoWaveform>;


fn main() -> Result<(), pa::Error> {
    rational_play();

//  read file
//  parse file
//  

    let normal_form_op: NormOp = Overlay { operations: vec![
            Sequence { operations: vec![
                AsIs, TransposeM {m: 5.0/4.0}, AsIs
            ]},
            Sequence { operations: vec![
                TransposeM {m: 5.0/4.0}, TransposeM {m: 8.0/5.0}, TransposeM {m: 5.0/4.0}
            ]},
            Sequence { operations: vec![
                Silence {m: 2.0}, TransposeM {m: 0.5}
            ]},
        ]};

    let composition = render(normal_form_op);

    let pa = pa::PortAudio::new()?;

    let mut output_stream = setup_portaudio_output(composition, &pa)?;
    output_stream.start()?;

    while let true = output_stream.is_active()? {}

    output_stream.stop()?;

    Ok(())
}

fn render(normal_form_op: NormOp) -> StereoWaveform {
    let sequences: Sequences = normal_form_op.get_operations().expect("Not in Normal Form");

    println!("\n ____Sequences____ \n{:?}", sequences);

//  NormOp -> NormEv
    let e = Event::init(200.0, 2.0, 0.0, 1.0);

    let mut norm_ev: NormEv = vec![];
    for sequence in sequences {
        norm_ev.push(sequence.apply(vec![e.clone()]))
    }

    println!("\n ____Creating Events____ \n{:?}", norm_ev);

    let mut vec_wav: VecWav = vec![];
    for mut vec_events in norm_ev {
        let mut osc = Oscillator::init(&get_default_app_settings());
        vec_wav.push(vec_events.render(&mut osc))
    }

    println!("____Rendering____");
    println!("Rendered {:?} waveforms", vec_wav.len());

    println!("\n____Combining Waveforms____");
    let mut result = StereoWaveform::new(0);
    for wav in vec_wav {
        result.l_buffer = sum_vec(&result.l_buffer, wav.l_buffer);
        result.r_buffer = sum_vec(&result.r_buffer, wav.r_buffer)
    }

    println!("...combined\n");

    result
}

fn sum_vec(a: &Vec<f32>, b: Vec<f32>) -> Vec<f32> {
    let vec_len = std::cmp::max(a.len(), b.len());
    let mut acc: Vec<f32> = vec![0.0; vec_len];
    for (i, e) in a.iter().zip_longest(&b).enumerate() {
        match e {
            itertools::EitherOrBoth::Both(v1, v2) => acc[i] = v1 + v2,
            itertools::EitherOrBoth::Left(e) => acc[i] = *e,
            itertools::EitherOrBoth::Right(e) => acc[i] = *e
        }
    }

    acc
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_render() {}
}
