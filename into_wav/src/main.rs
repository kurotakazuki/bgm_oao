use floaout::bub::BubReader;
use floaout::coord::BubFnsCoord;
use floaout::oao::OaoReader;
use floaout::wav::{WavMetadata, WavWriter};
use floaout::LpcmKind;
use std::io::Result;

const BUBS_DATA: [&[u8]; 35] = [
    include_bytes!("../../output/hi_hat_first.bub"),
    include_bytes!("../../output/open_hi_hat_first.L.bub"),
    include_bytes!("../../output/open_hi_hat_first.R.bub"),
    include_bytes!("../../output/open_hi_hat_rev.bub"),
    include_bytes!("../../output/hi_hat_last.bub"),
    include_bytes!("../../output/kick_last.bub"),
    include_bytes!("../../output/phan_last.bub"),
    include_bytes!("../../output/phan2_last.bub"),
    include_bytes!("../../output/pluck_chord_last.bub"),
    include_bytes!("../../output/sine_chord_last.bub"),
    include_bytes!("../../output/guitar_last.bub"),
    include_bytes!("../../output/bass.bub"),
    include_bytes!("../../output/hi_hat.bub"),
    include_bytes!("../../output/kick.bub"),
    include_bytes!("../../output/mini_bass.bub"),
    include_bytes!("../../output/sine_chord.bub"),
    include_bytes!("../../output/stereo_drum.L.bub"),
    include_bytes!("../../output/stereo_drum.R.bub"),
    include_bytes!("../../output/stereo_guitar.L.bub"),
    include_bytes!("../../output/stereo_guitar.R.bub"),
    include_bytes!("../../output/stereo_pha-n.L.bub"),
    include_bytes!("../../output/stereo_pha-n.R.bub"),
    include_bytes!("../../output/stereo_phan.L.bub"),
    include_bytes!("../../output/stereo_phan.R.bub"),
    include_bytes!("../../output/stereo_phan2.L.bub"),
    include_bytes!("../../output/stereo_phan2.R.bub"),
    include_bytes!("../../output/stereo_pluck_chord.L.bub"),
    include_bytes!("../../output/stereo_pluck_chord.R.bub"),
    include_bytes!("../../output/stereo_po.L.bub"),
    include_bytes!("../../output/stereo_po.R.bub"),
    include_bytes!("../../output/stereo_toro.L.bub"),
    include_bytes!("../../output/stereo_toro.R.bub"),
    include_bytes!("../../output/sub_bass1.bub"),
    include_bytes!("../../output/sub_bass2.bub"),
    include_bytes!("../../output/xylophone.bub"),
];

fn main() -> Result<()> {
    //
    //
    // Floaout into wav
    let speakers_absolute_coord: &[BubFnsCoord] = &[
        (-1.0, -1.0, -1.0),
        (-1.0, -1.0, 1.0),
        (-1.0, 1.0, -1.0),
        (-1.0, 1.0, 1.0),
        (1.0, -1.0, -1.0),
        (1.0, -1.0, 1.0),
        (1.0, 1.0, -1.0),
        (1.0, 1.0, 1.0),
    ]
    .map(Into::into);
    let oao_reader = OaoReader::new(
        include_bytes!("../../output/bgm.oao").as_ref(),
        speakers_absolute_coord.into(),
    )?;

    let frames = oao_reader.metadata.frames();
    let wav_metadata = WavMetadata::new(frames, LpcmKind::F32LE, 1, 48000.0, vec![]);

    // each bubble
    let bub_frame_readers = BUBS_DATA
        .into_iter()
        .map(|bub| {
            let bub_reader =
                BubReader::new(bub, speakers_absolute_coord.into()).expect("valid bubble metadata");
            // if bub_reader.metadata.bub_id.rgb == None {
            //     let r = js_sys::Math::random() as f32;
            //     let g = js_sys::Math::random() as f32;
            //     let b = js_sys::Math::random() as f32;
            //     bub_reader.metadata.bub_id.rgb = Some((r, g, b).into());
            // }

            unsafe { bub_reader.into_bub_frame_reader::<f32>(None) }
        })
        .collect();

    let oao_frame_reader =
        unsafe { oao_reader.into_oao_frame_reader::<&[u8], f32>(bub_frame_readers, None) };
    let mut wav_frame_writers = vec![];
    let wav_len = speakers_absolute_coord.len();
    for i in 0..wav_len {
        let (x, y, z) = speakers_absolute_coord[i].into();
        let wav_writer = WavWriter::create(format!("{}_{}_{}.wav", x, y, z), wav_metadata.clone())?;
        wav_frame_writers.push(unsafe { wav_writer.into_wav_frame_writer() });
    }

    for frame in oao_frame_reader {
        let frame = frame?;
        for i in 0..wav_len {
            wav_frame_writers[i].write_frame(vec![frame.0[i]].into())?;
        }
    }

    Ok(())
}
