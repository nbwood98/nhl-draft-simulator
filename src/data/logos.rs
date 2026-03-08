use crate::data::nhl_api::TeamData;

/// Width/height of the braille art in terminal cells.
/// Braille characters encode a 2×4 dot grid per cell,
/// so we rasterize to (BRAILLE_W*2) × (BRAILLE_H*4) pixels.
const BRAILLE_W: u32 = 9;
const BRAILLE_H: u32 = 5;

const PIXEL_W: u32 = BRAILLE_W * 2;  // 18
const PIXEL_H: u32 = BRAILLE_H * 4;  // 20

/// Downloads each team's SVG logo, rasterizes it, and converts to braille art.
/// Returns one `Vec<String>` per team (each string is one row of braille characters).
pub fn fetch_logos(teams: &[TeamData]) -> Vec<Vec<String>> {
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(5))
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .ok();

    teams
        .iter()
        .map(|team| {
            client
                .as_ref()
                .and_then(|c| fetch_single_logo(c, &team.logo_url).ok())
                .unwrap_or_else(placeholder_art)
        })
        .collect()
}

fn fetch_single_logo(
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let svg_bytes = client.get(url).send()?.bytes()?;
    svg_to_braille(&svg_bytes)
}

fn svg_to_braille(svg_data: &[u8]) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Parse SVG
    let options = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_data(svg_data, &options)?;

    let svg_size = tree.size();
    let fit_transform = resvg::tiny_skia::Transform::from_scale(
        PIXEL_W as f32 / svg_size.width(),
        PIXEL_H as f32 / svg_size.height(),
    );

    let mut pixmap =
        resvg::tiny_skia::Pixmap::new(PIXEL_W, PIXEL_H).ok_or("failed to create pixmap")?;

    // Render with transparent background
    resvg::render(&tree, fit_transform, &mut pixmap.as_mut());

    // Convert to grayscale luminance buffer
    let data = pixmap.data(); // RGBA
    let mut luma = vec![0u8; (PIXEL_W * PIXEL_H) as usize];
    for i in 0..luma.len() {
        let r = data[i * 4] as f32;
        let g = data[i * 4 + 1] as f32;
        let b = data[i * 4 + 2] as f32;
        let a = data[i * 4 + 3] as f32 / 255.0;
        // Luminance weighted by alpha (transparent = dark)
        luma[i] = ((0.299 * r + 0.587 * g + 0.114 * b) * a) as u8;
    }

    // Threshold: use Otsu-like simple midpoint
    let threshold = 80u8;

    // Map 2×4 pixel blocks to braille characters
    let mut rows: Vec<String> = Vec::with_capacity(BRAILLE_H as usize);
    for by in 0..BRAILLE_H {
        let mut row = String::with_capacity(BRAILLE_W as usize);
        for bx in 0..BRAILLE_W {
            let mut dots: u8 = 0;
            // Braille dot positions (col, row) → bit:
            // (0,0)→0  (1,0)→3
            // (0,1)→1  (1,1)→4
            // (0,2)→2  (1,2)→5
            // (0,3)→6  (1,3)→7
            let offsets: [(u32, u32, u8); 8] = [
                (0, 0, 0),
                (0, 1, 1),
                (0, 2, 2),
                (0, 3, 6),
                (1, 0, 3),
                (1, 1, 4),
                (1, 2, 5),
                (1, 3, 7),
            ];
            for (dx, dy, bit) in offsets {
                let px = bx * 2 + dx;
                let py = by * 4 + dy;
                if px < PIXEL_W && py < PIXEL_H {
                    let idx = (py * PIXEL_W + px) as usize;
                    if luma[idx] > threshold {
                        dots |= 1 << bit;
                    }
                }
            }
            row.push(char::from_u32(0x2800 + dots as u32).unwrap_or(' '));
        }
        rows.push(row);
    }

    Ok(rows)
}

pub fn placeholder_art() -> Vec<String> {
    vec![
        "⣀⣤⣤⣤⣤⣤⣀  ".into(),
        "⣿⣿⣿⣿⣿⣿⣿ ".into(),
        "⣿⣿⣿⣿⣿⣿⣿ ".into(),
        "⣿⣿⣿⣿⣿⣿⣿ ".into(),
        "⠉⠛⠛⠛⠛⠛⠉  ".into(),
    ]
}

