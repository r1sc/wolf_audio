use libwolf::{gr, vswap::VSWAPArchive, wl6_igrab};
use minifb::{Key, Window, WindowOptions};
use std::{env::args, fs::File, io::BufReader};

const GAMEPAL: [u8; 768] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x2A, 0x00, 0x2A, 0x00, 0x00, 0x2A, 0x2A, 0x2A, 0x00, 0x00, 0x2A,
    0x00, 0x2A, 0x2A, 0x15, 0x00, 0x2A, 0x2A, 0x2A, 0x15, 0x15, 0x15, 0x15, 0x15, 0x3F, 0x15, 0x3F,
    0x15, 0x15, 0x3F, 0x3F, 0x3F, 0x15, 0x15, 0x3F, 0x15, 0x3F, 0x3F, 0x3F, 0x15, 0x3F, 0x3F, 0x3F,
    0x3B, 0x3B, 0x3B, 0x37, 0x37, 0x37, 0x34, 0x34, 0x34, 0x30, 0x30, 0x30, 0x2D, 0x2D, 0x2D, 0x2A,
    0x2A, 0x2A, 0x26, 0x26, 0x26, 0x23, 0x23, 0x23, 0x1F, 0x1F, 0x1F, 0x1C, 0x1C, 0x1C, 0x19, 0x19,
    0x19, 0x15, 0x15, 0x15, 0x12, 0x12, 0x12, 0x0E, 0x0E, 0x0E, 0x0B, 0x0B, 0x0B, 0x08, 0x08, 0x08,
    0x3F, 0x00, 0x00, 0x3B, 0x00, 0x00, 0x38, 0x00, 0x00, 0x35, 0x00, 0x00, 0x32, 0x00, 0x00, 0x2F,
    0x00, 0x00, 0x2C, 0x00, 0x00, 0x29, 0x00, 0x00, 0x26, 0x00, 0x00, 0x22, 0x00, 0x00, 0x1F, 0x00,
    0x00, 0x1C, 0x00, 0x00, 0x19, 0x00, 0x00, 0x16, 0x00, 0x00, 0x13, 0x00, 0x00, 0x10, 0x00, 0x00,
    0x3F, 0x36, 0x36, 0x3F, 0x2E, 0x2E, 0x3F, 0x27, 0x27, 0x3F, 0x1F, 0x1F, 0x3F, 0x17, 0x17, 0x3F,
    0x10, 0x10, 0x3F, 0x08, 0x08, 0x3F, 0x00, 0x00, 0x3F, 0x2A, 0x17, 0x3F, 0x26, 0x10, 0x3F, 0x22,
    0x08, 0x3F, 0x1E, 0x00, 0x39, 0x1B, 0x00, 0x33, 0x18, 0x00, 0x2D, 0x15, 0x00, 0x27, 0x13, 0x00,
    0x3F, 0x3F, 0x36, 0x3F, 0x3F, 0x2E, 0x3F, 0x3F, 0x27, 0x3F, 0x3F, 0x1F, 0x3F, 0x3E, 0x17, 0x3F,
    0x3D, 0x10, 0x3F, 0x3D, 0x08, 0x3F, 0x3D, 0x00, 0x39, 0x36, 0x00, 0x33, 0x31, 0x00, 0x2D, 0x2B,
    0x00, 0x27, 0x27, 0x00, 0x21, 0x21, 0x00, 0x1C, 0x1B, 0x00, 0x16, 0x15, 0x00, 0x10, 0x10, 0x00,
    0x34, 0x3F, 0x17, 0x31, 0x3F, 0x10, 0x2D, 0x3F, 0x08, 0x28, 0x3F, 0x00, 0x24, 0x39, 0x00, 0x20,
    0x33, 0x00, 0x1D, 0x2D, 0x00, 0x18, 0x27, 0x00, 0x36, 0x3F, 0x36, 0x2F, 0x3F, 0x2E, 0x27, 0x3F,
    0x27, 0x20, 0x3F, 0x1F, 0x18, 0x3F, 0x17, 0x10, 0x3F, 0x10, 0x08, 0x3F, 0x08, 0x00, 0x3F, 0x00,
    0x00, 0x3F, 0x00, 0x00, 0x3B, 0x00, 0x00, 0x38, 0x00, 0x00, 0x35, 0x00, 0x01, 0x32, 0x00, 0x01,
    0x2F, 0x00, 0x01, 0x2C, 0x00, 0x01, 0x29, 0x00, 0x01, 0x26, 0x00, 0x01, 0x22, 0x00, 0x01, 0x1F,
    0x00, 0x01, 0x1C, 0x00, 0x01, 0x19, 0x00, 0x01, 0x16, 0x00, 0x01, 0x13, 0x00, 0x01, 0x10, 0x00,
    0x36, 0x3F, 0x3F, 0x2E, 0x3F, 0x3F, 0x27, 0x3F, 0x3F, 0x1F, 0x3F, 0x3E, 0x17, 0x3F, 0x3F, 0x10,
    0x3F, 0x3F, 0x08, 0x3F, 0x3F, 0x00, 0x3F, 0x3F, 0x00, 0x39, 0x39, 0x00, 0x33, 0x33, 0x00, 0x2D,
    0x2D, 0x00, 0x27, 0x27, 0x00, 0x21, 0x21, 0x00, 0x1C, 0x1C, 0x00, 0x16, 0x16, 0x00, 0x10, 0x10,
    0x17, 0x2F, 0x3F, 0x10, 0x2C, 0x3F, 0x08, 0x2A, 0x3F, 0x00, 0x27, 0x3F, 0x00, 0x23, 0x39, 0x00,
    0x1F, 0x33, 0x00, 0x1B, 0x2D, 0x00, 0x17, 0x27, 0x36, 0x36, 0x3F, 0x2E, 0x2F, 0x3F, 0x27, 0x27,
    0x3F, 0x1F, 0x20, 0x3F, 0x17, 0x18, 0x3F, 0x10, 0x10, 0x3F, 0x08, 0x09, 0x3F, 0x00, 0x01, 0x3F,
    0x00, 0x00, 0x3F, 0x00, 0x00, 0x3B, 0x00, 0x00, 0x38, 0x00, 0x00, 0x35, 0x00, 0x00, 0x32, 0x00,
    0x00, 0x2F, 0x00, 0x00, 0x2C, 0x00, 0x00, 0x29, 0x00, 0x00, 0x26, 0x00, 0x00, 0x22, 0x00, 0x00,
    0x1F, 0x00, 0x00, 0x1C, 0x00, 0x00, 0x19, 0x00, 0x00, 0x16, 0x00, 0x00, 0x13, 0x00, 0x00, 0x10,
    0x0A, 0x0A, 0x0A, 0x3F, 0x38, 0x0D, 0x3F, 0x35, 0x09, 0x3F, 0x33, 0x06, 0x3F, 0x30, 0x02, 0x3F,
    0x2D, 0x00, 0x2D, 0x08, 0x3F, 0x2A, 0x00, 0x3F, 0x26, 0x00, 0x39, 0x20, 0x00, 0x33, 0x1D, 0x00,
    0x2D, 0x18, 0x00, 0x27, 0x14, 0x00, 0x21, 0x11, 0x00, 0x1C, 0x0D, 0x00, 0x16, 0x0A, 0x00, 0x10,
    0x3F, 0x36, 0x3F, 0x3F, 0x2E, 0x3F, 0x3F, 0x27, 0x3F, 0x3F, 0x1F, 0x3F, 0x3F, 0x17, 0x3F, 0x3F,
    0x10, 0x3F, 0x3F, 0x08, 0x3F, 0x3F, 0x00, 0x3F, 0x38, 0x00, 0x39, 0x32, 0x00, 0x33, 0x2D, 0x00,
    0x2D, 0x27, 0x00, 0x27, 0x21, 0x00, 0x21, 0x1B, 0x00, 0x1C, 0x16, 0x00, 0x16, 0x10, 0x00, 0x10,
    0x3F, 0x3A, 0x37, 0x3F, 0x38, 0x34, 0x3F, 0x36, 0x31, 0x3F, 0x35, 0x2F, 0x3F, 0x33, 0x2C, 0x3F,
    0x31, 0x29, 0x3F, 0x2F, 0x27, 0x3F, 0x2E, 0x24, 0x3F, 0x2C, 0x20, 0x3F, 0x29, 0x1C, 0x3F, 0x27,
    0x18, 0x3C, 0x25, 0x17, 0x3A, 0x23, 0x16, 0x37, 0x22, 0x15, 0x34, 0x20, 0x14, 0x32, 0x1F, 0x13,
    0x2F, 0x1E, 0x12, 0x2D, 0x1C, 0x11, 0x2A, 0x1A, 0x10, 0x28, 0x19, 0x0F, 0x27, 0x18, 0x0E, 0x24,
    0x17, 0x0D, 0x22, 0x16, 0x0C, 0x20, 0x14, 0x0B, 0x1D, 0x13, 0x0A, 0x1B, 0x12, 0x09, 0x17, 0x10,
    0x08, 0x15, 0x0F, 0x07, 0x12, 0x0E, 0x06, 0x10, 0x0C, 0x06, 0x0E, 0x0B, 0x05, 0x0A, 0x08, 0x03,
    0x18, 0x00, 0x19, 0x00, 0x19, 0x19, 0x00, 0x18, 0x18, 0x00, 0x00, 0x07, 0x00, 0x00, 0x0B, 0x0C,
    0x09, 0x04, 0x12, 0x00, 0x12, 0x14, 0x00, 0x14, 0x00, 0x00, 0x0D, 0x07, 0x07, 0x07, 0x13, 0x13,
    0x13, 0x17, 0x17, 0x17, 0x10, 0x10, 0x10, 0x0C, 0x0C, 0x0C, 0x0D, 0x0D, 0x0D, 0x36, 0x3D, 0x3D,
    0x2E, 0x3A, 0x3A, 0x27, 0x37, 0x37, 0x1D, 0x32, 0x32, 0x12, 0x30, 0x30, 0x08, 0x2D, 0x2D, 0x08,
    0x2C, 0x2C, 0x00, 0x29, 0x29, 0x00, 0x26, 0x26, 0x00, 0x23, 0x23, 0x00, 0x21, 0x21, 0x00, 0x1F,
    0x1F, 0x00, 0x1E, 0x1E, 0x00, 0x1D, 0x1D, 0x00, 0x1C, 0x1C, 0x00, 0x1B, 0x1B, 0x26, 0x00, 0x22,
];

fn main() {
    let asset_number = args()
        .nth(1)
        .expect("usage: wolf_audio <music number>")
        .parse::<usize>()
        .expect("<music number> must be a number");

    let mut palette_u32 = vec![0; 256];
    let brightness = 2;

    for i in 0..256 {
        let r = GAMEPAL[i * 3] as u32;
        let g = GAMEPAL[i * 3 + 1] as u32;
        let b = GAMEPAL[i * 3 + 2] as u32;

        palette_u32[i] = (r << brightness << 16) | (g << brightness << 8) | b << brightness;
    }

    let mut screen_buffer: Vec<u32> = vec![0; 320 * 200];

    let wolf_base_path = r"c:\classic\wolf3d";

    let mut gr = gr::GrArchive::new(wolf_base_path);
    let chunk_index = wl6_igrab::GraphicNums::TITLEPIC as usize;
    let title_pic = gr.expand_chunk(chunk_index);

    let pic_size = gr.get_pic_size_for_chunk(chunk_index);
    let quater_width = pic_size.width / 4;
    let plane_size = (pic_size.width as usize * pic_size.height as usize) / 4;
    let mut i = 0;
    for y in 0..pic_size.height as usize {
        for x in 0..quater_width as usize {
            let dst_index = y * 320 + x * 4;
            screen_buffer[dst_index + 0] = palette_u32[title_pic[i] as usize];
            screen_buffer[dst_index + 1] = palette_u32[title_pic[i + plane_size] as usize];
            screen_buffer[dst_index + 2] = palette_u32[title_pic[i + plane_size * 2] as usize];
            screen_buffer[dst_index + 3] = palette_u32[title_pic[i + plane_size * 3] as usize];
            i += 1;
        }
    }

    let mut reader = BufReader::new(File::open(format!("{}/vswap.wl6", wolf_base_path)).unwrap());
    let vswap = VSWAPArchive::open(&mut reader).unwrap();

    let mut current_sprite = 0;

    vswap.rasterize_wall(18, &palette_u32, &mut screen_buffer);
    vswap.rasterize_sprite(54, &palette_u32, &mut screen_buffer);

    let output_sample_rate = 44100;
    let num_streaming_buffers = 4;
    let music_buffer_size = 12000;
    let num_channels = 2; // Stereo

    let mut imf = libwolf::imf::Imf::new(wolf_base_path, asset_number, output_sample_rate).unwrap();

    let mut mixer = libwolf::mixer::Mixer::new(num_streaming_buffers);
    let mut music_buffer: Vec<i16> = vec![0; music_buffer_size * num_channels as usize];

    for _ in 0..num_streaming_buffers {
        imf.fill_audio_buffer(&mut music_buffer, num_channels).unwrap();
        mixer.queue_music_data(output_sample_rate, num_channels, &music_buffer);
    }

    let pcm_sound = mixer.load_raw_pcm(7000, &vswap.raw_pcm_chunks[asset_number]);
    mixer.play_pcm_buffer(&pcm_sound, 0.2, true);



    let scale = 2;
    let mut window = Window::new(
        "Test - ESC to exit",
        320 * scale,
        240 * scale,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.set_target_fps(60);

    while window.is_open() {
        if window.is_key_pressed(Key::Right, minifb::KeyRepeat::Yes)
            && current_sprite < vswap.sprite_chunks.len() - 1
        {
            current_sprite += 1;
            screen_buffer.fill(0);
            vswap.rasterize_sprite(current_sprite, &palette_u32, &mut screen_buffer);
        } else if window.is_key_pressed(Key::Left, minifb::KeyRepeat::Yes) && current_sprite > 0 {
            current_sprite -= 1;
            screen_buffer.fill(0);
            vswap.rasterize_sprite(current_sprite, &palette_u32, &mut screen_buffer);
        }

        window.update_with_buffer(&screen_buffer, 320, 200).unwrap();

        // Process music
        mixer.unqueue_processed_buffers();

        if mixer.get_num_empty_music_buffers() > 0 {
            // mixer.print_buffer_queue();

            imf.fill_audio_buffer(&mut music_buffer, num_channels).unwrap();
            mixer.queue_music_data(output_sample_rate, num_channels, &music_buffer);
        }
    }
}
