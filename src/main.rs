use floaout::bub::{BubFnsBlock, BubMetadata, BubSampleKind, BubWriter};
use floaout::oao::{BubInOao, OaoMetadata, OaoWriter};
use floaout::wav::WavReader;
use floaout::LpcmKind;
use std::io::Result;

fn main() -> Result<()> {
    let bubs = vec![
        ("bass", "Bass", "0 0 0 x<0 0.45 0 0 0 0<=x 0.55"),
        ("hi_hat", "Hi Hat", "0 0 0 x<0 0.525 0 0 0 0<=x 0.475"),
        ("hi_hat_last", "Hi Hat Last", "0 0 0 0==0 1"),
        (
            "hi_hat_first",
            "Hi Hat First",
            "0 0 0 x<0 0.525 0 0 0 0<=x 0.475",
        ),
        ("kick", "Kick", "0 0 0 x<0 0.55 0 0 0 0<=x 0.45"),
        ("kick_last", "Kick Last", "0 0 0 x<0 0.55 0 0 0 0<=x 0.45"),
        ("mini_bass", "Mini Bass", "0 0 0 0==0 0.5*sin(X-N/S)+0.5"),
        ("open_hi_hat_first.L", "Open Hi Hat First L", "0 0 0 x<0 1"),
        ("open_hi_hat_first.R", "Open Hi Hat First R", "0 0 0 0<x 1"),
        ("open_hi_hat_rev", "Open Hi Hat Rev", "0 0 0 0==0 1"),
        ("phan_last", "Phan Last", "0 0 0 0==0 0.5*cos(X-5*N/S)+0.5"),
        (
            "phan2_last",
            "Phan2 Last",
            "0 0 0 0==0 0.5*cos(X+5*N/S)+0.5",
        ),
        (
            "pluck_chord_last",
            "Pluck Chord Last",
            "0 0 0 0==0 0.5*cos(X+8*N/S)+0.5",
        ),
        (
            "sine_chord",
            "Sine Chord",
            "0 0 0 0==0 0.25*cos(X+8*N/S)+0.75",
        ),
        // ("sine_chord", "Sine Chord", "0 0 0 0==0 0.5*cos(X+8*N/S)+0.5"),
        (
            "sine_chord_last",
            "Sine Chord Last",
            "0 0 0 0==0 (n/F-1/8)*x+1/2",
        ),
        ("guitar_last", "Guitar Last", "0 0 0 0==0 (n/F-1/4)*x+1/2"),
        ("stereo_drum.L", "Drum L", "0 0 0 0==0 (n/F-1/2)*x+1/2"),
        ("stereo_drum.R", "Drum R", "0 0 0 0==0 (1/2-n/F)*x+1/2"),
        // ("stereo_guitar.L", "Guitar L", "0 0 0 0==0 0.5*cos(X+N/S)+0.5"),
        // ("stereo_guitar.R", "Guitar R", "0 0 0 0==0 0.5*cos(X-N/S)+0.5"),
        ("stereo_guitar.L", "Guitar L", "0 0 0 x<0 1"),
        ("stereo_guitar.R", "Guitar R", "0 0 0 0<x 1"),
        ("stereo_pha-n.L", "Pha-n L", "0 0 0 0==0 0.5*cos(X+N/S)+0.5"),
        ("stereo_pha-n.R", "Pha-n R", "0 0 0 0==0 0.5*cos(X-N/S)+0.5"),
        ("stereo_phan.L", "Phan L", "0 0 0 0==0 0.5*cos(X-N/S)+0.5"),
        ("stereo_phan.R", "Phan R", "0 0 0 0==0 0.5*cos(X+N/S)+0.5"),
        ("stereo_phan2.L", "Phan2 L", "0 0 0 0==0 0.5*cos(X-N/S)+0.5"),
        ("stereo_phan2.R", "Phan2 R", "0 0 0 0==0 0.5*cos(X+N/S)+0.5"),
        (
            "stereo_pluck_chord.L",
            "Pluck Chord L",
            "0 0 0 X<0 1.5/(0.5*PI)*E^(-(x^2+y^2)/0.5)",
        ),
        (
            "stereo_pluck_chord.R",
            "Pluck Chord R",
            "0 0 0 0<X 1.5/(0.5*PI)*E^(-(x^2+y^2)/0.5)",
        ),
        ("stereo_po.L", "Po L", "0 0 0 0==0 0.5*cos(X-N/S)+0.5"),
        ("stereo_po.R", "Po R", "0 0 0 0==0 0.5*cos(X-N/S+PI)+0.5"),
        ("stereo_toro.L", "Toro L", "0 0 0 0==0 0.5*cos(X-N/S)+0.5"),
        (
            "stereo_toro.R",
            "Toro R",
            "0 0 0 0==0 0.5*cos(X-N/S+PI)+0.5",
        ),
        ("sub_bass1", "Sub Bass1", "0 0 0 x<0 0.48 0 0 0 0<=x 0.52"),
        ("sub_bass2", "Sub Bass2", "0 0 0 x<0 0.52 0 0 0 0<=x 0.48"),
        (
            "xylophone",
            "Xylophone",
            "0 0 0 x<0 0.5*cos(X+2*N/S)+0.5 0 0 0 0<=x 0.5*cos(X-3*N/S)+0.5",
        ),
    ];

    for bub in bubs {
        let wav_reader = WavReader::open(format!("{}.wav", bub.0))?;
        let frames = wav_reader.metadata.frames();
        let wav_frame_reader = unsafe { wav_reader.into_wav_frame_reader::<f32>() };
        let metadata = BubMetadata::new(
            frames,
            1,
            48000.0,
            LpcmKind::F32LE,
            BubSampleKind::Lpcm,
            String::from(bub.1),
        );
        // BubFnsBlock
        let mut samples = Vec::with_capacity(metadata.frames() as usize);
        for frame in wav_frame_reader {
            let frame = frame?;
            samples.push(frame.0[0]);
        }
        let bub_fns_block = BubFnsBlock::Lpcm {
            bub_fns: bub.2.as_bytes(),
            next_head_relative_frame: None,
            samples,
        };
        // Write
        let bub_writer = BubWriter::create(format!("{}.bub", bub.0), metadata)?;
        let mut bub_frame_writer = unsafe { bub_writer.into_bub_frame_writer::<f32>() };
        bub_frame_writer.write_head_to_less_than_next_head_or_ended(bub_fns_block)?;
    }

    let two = 48000 * 2;
    let first: &[u64] = &[1];
    let most: &[u64] = &[two + 1, two + 768_000 + 1];
    let last: &[u64] = &[two + 768_000 * 2 + 1];
    // Create Oao
    let bubs_data = vec![
        ("hi_hat_first", first),
        ("open_hi_hat_first.L", first),
        ("open_hi_hat_first.R", first),
        ("open_hi_hat_rev", first),
        ("hi_hat_last", last),
        ("kick_last", last),
        ("phan_last", last),
        ("phan2_last", last),
        ("pluck_chord_last", last),
        ("sine_chord_last", last),
        ("guitar_last", last),
        ("bass", most),
        ("hi_hat", most),
        ("kick", most),
        ("mini_bass", most),
        ("sine_chord", most),
        ("stereo_drum.L", most),
        ("stereo_drum.R", most),
        ("stereo_guitar.L", most),
        ("stereo_guitar.R", most),
        ("stereo_pha-n.L", most),
        ("stereo_pha-n.R", most),
        ("stereo_phan.L", most),
        ("stereo_phan.R", most),
        ("stereo_phan2.L", most),
        ("stereo_phan2.R", most),
        ("stereo_pluck_chord.L", most),
        ("stereo_pluck_chord.R", most),
        ("stereo_po.L", most),
        ("stereo_po.R", most),
        ("stereo_toro.L", most),
        ("stereo_toro.R", most),
        ("sub_bass1", most),
        ("sub_bass2", most),
        ("xylophone", most),
    ];
    let mut bubs = Vec::new();
    for bub in bubs_data {
        bubs.push(BubInOao {
            file_name: bub.0.into(),
            starting_frames: bub.1.to_vec().into(),
        });
    }
    let oao_metadata = OaoMetadata::new(
        two + 768_000 * 2 + 48000 * 2,
        48000.0,
        LpcmKind::F32LE,
        String::from("BGM"),
        String::from("Kazuki Kurota"),
        bubs,
    );
    OaoWriter::create("bgm.oao", oao_metadata)?;

    Ok(())
}
