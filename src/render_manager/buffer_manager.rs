use crate::instrument::StereoWaveform;

#[derive(Clone, Debug)]
pub struct Buffer {
    pub stereo_waveform: StereoWaveform,
    pub write_idx: usize,
    pub read_idx: usize,
}

/// This assumes that all buffers are the same size
impl Buffer {
    pub fn init() -> Self {
        Self {
            stereo_waveform: StereoWaveform::new(0),
            write_idx: 0,
            read_idx: 0,
        }
    }
    pub fn write(&mut self, stereo_waveform: StereoWaveform) {
        self.stereo_waveform.append(stereo_waveform);
        self.write_idx += 1;
    }

    pub fn read(&mut self, buffer_size: usize) -> Option<StereoWaveform> {
        let sw = self.stereo_waveform.get_buffer(self.read_idx, buffer_size);
        if sw.is_some() {
            self.read_idx += 1;
        };
        sw
    }
}

#[derive(Clone, Debug)]
pub struct BufferManager {
    pub buffers: [Option<Buffer>; 2],
    renderer_write_idx: usize,
    buffer_idx: usize,
    write_idx: usize,
    read_idx: usize,
}

impl BufferManager {
    pub const fn init_silent() -> Self {
        Self {
            buffers: [None, None],
            renderer_write_idx: 0,
            buffer_idx: 0,
            write_idx: 0,
            read_idx: 0,
        }
    }

    pub fn inc_buffer(&mut self) {
        self.buffer_idx = (self.buffer_idx + 1) % 2;
    }

    pub fn inc_render_write_buffer(&mut self) {
        self.renderer_write_idx = (self.renderer_write_idx + 1) % 2;
    }

    pub fn current_buffer(&mut self) -> &mut Option<Buffer> {
        &mut self.buffers[self.buffer_idx]
    }

    pub fn current_render_write_buffer(&mut self) -> &mut Option<Buffer> {
        &mut self.buffers[self.renderer_write_idx]
    }

    pub fn next_buffer(&mut self) -> &mut Option<Buffer> {
        &mut self.buffers[(self.buffer_idx + 1) % 2]
    }

    pub fn exists_new_buffer(&mut self) -> bool {
        self.next_buffer().is_some()
    }

    pub fn read(&mut self, buffer_size: usize) -> Option<StereoWaveform> {
        let next = self.exists_new_buffer();
        let current = self.current_buffer();

        match current {
            Some(buffer) => {
                let mut sw = buffer.read(buffer_size);

                if next {
                    if let Some(s) = sw.as_mut() {
                        s.fade_out()
                    }

                    *current = None;
                    self.inc_buffer();
                }
                sw
            }
            None => {
                if next {
                    self.inc_buffer();
                    self.read(buffer_size)
                } else {
                    None
                }
            }
        }
    }

    pub fn write(&mut self, stereo_waveform: StereoWaveform) {
        let current = self.current_render_write_buffer();
        match current {
            Some(buffer) => buffer.write(stereo_waveform),
            None => {
                let mut new_buffer = Buffer::init();
                new_buffer.write(stereo_waveform);
                *current = Some(new_buffer);
            }
        }
    }
}