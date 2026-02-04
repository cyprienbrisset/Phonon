use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};
use std::path::Path;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub struct AudioDecoder;

impl AudioDecoder {
    /// Decode audio file to f32 samples at 16kHz mono
    pub fn decode_file(path: &Path) -> Result<(Vec<f32>, u32), String> {
        let file =
            std::fs::File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;

        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        }

        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();
        let decoder_opts = DecoderOptions::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &format_opts, &metadata_opts)
            .map_err(|e| format!("Failed to probe format: {}", e))?;

        let mut format = probed.format;

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
            .ok_or("No supported audio track found")?;

        let track_id = track.id;
        let codec_params = track.codec_params.clone();

        let mut decoder = symphonia::default::get_codecs()
            .make(&codec_params, &decoder_opts)
            .map_err(|e| format!("Failed to create decoder: {}", e))?;

        let sample_rate = codec_params.sample_rate.unwrap_or(44100);
        let channels = codec_params.channels.map(|c| c.count()).unwrap_or(1);

        let mut all_samples: Vec<f32> = Vec::new();

        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(symphonia::core::errors::Error::IoError(ref e))
                    if e.kind() == std::io::ErrorKind::UnexpectedEof =>
                {
                    break;
                }
                Err(symphonia::core::errors::Error::ResetRequired) => {
                    // Handle reset required by resetting the decoder
                    decoder.reset();
                    continue;
                }
                Err(e) => return Err(format!("Decode error: {}", e)),
            };

            if packet.track_id() != track_id {
                continue;
            }

            match decoder.decode(&packet) {
                Ok(decoded) => {
                    let spec = *decoded.spec();
                    let duration = decoded.capacity() as u64;

                    let mut sample_buf = SampleBuffer::<f32>::new(duration, spec);
                    sample_buf.copy_interleaved_ref(decoded);

                    let samples = sample_buf.samples();

                    // Convert to mono if needed
                    if channels > 1 {
                        for chunk in samples.chunks(channels) {
                            let mono: f32 = chunk.iter().sum::<f32>() / channels as f32;
                            all_samples.push(mono);
                        }
                    } else {
                        all_samples.extend_from_slice(samples);
                    }
                }
                Err(symphonia::core::errors::Error::DecodeError(_)) => {
                    // Skip decode errors
                    continue;
                }
                Err(e) => return Err(format!("Decode error: {}", e)),
            }
        }

        // Resample to 16kHz if needed
        let target_rate = 16000u32;
        if sample_rate != target_rate {
            all_samples = Self::resample(&all_samples, sample_rate, target_rate)?;
        }

        Ok((all_samples, target_rate))
    }

    fn resample(samples: &[f32], from_rate: u32, to_rate: u32) -> Result<Vec<f32>, String> {
        if samples.is_empty() {
            return Ok(Vec::new());
        }

        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };

        let mut resampler = SincFixedIn::<f32>::new(
            to_rate as f64 / from_rate as f64,
            2.0,
            params,
            samples.len(),
            1,
        )
        .map_err(|e| format!("Failed to create resampler: {}", e))?;

        let waves_in = vec![samples.to_vec()];
        let waves_out = resampler
            .process(&waves_in, None)
            .map_err(|e| format!("Resample error: {}", e))?;

        Ok(waves_out.into_iter().next().unwrap_or_default())
    }

    /// Get audio file duration in seconds without fully decoding
    pub fn get_duration(path: &Path) -> Result<f32, String> {
        let (samples, rate) = Self::decode_file(path)?;
        Ok(samples.len() as f32 / rate as f32)
    }

    /// Check if file format is supported
    pub fn is_supported(path: &Path) -> bool {
        match path.extension().and_then(|e| e.to_str()) {
            Some(ext) => matches!(
                ext.to_lowercase().as_str(),
                "wav" | "mp3" | "m4a" | "aac" | "flac" | "ogg" | "webm"
            ),
            None => false,
        }
    }

    /// Get list of supported formats
    pub fn supported_formats() -> Vec<&'static str> {
        vec!["wav", "mp3", "m4a", "aac", "flac", "ogg", "webm"]
    }
}
