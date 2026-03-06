use rand::seq::SliceRandom;
use rand::thread_rng;
use raylib::prelude::*;

// ─── Constants ───────────────────────────────────────────────────────────────

const SCREEN_W: i32 = 1280;
const SCREEN_H: i32 = 800;
const TILE_W: f32 = 56.0;
const TILE_H: f32 = 72.0;
const TILE_DEPTH: f32 = 6.0; // visual z-offset per layer
const GRID_UNIT_X: f32 = TILE_W * 0.5; // half-tile for grid snapping
const GRID_UNIT_Y: f32 = TILE_H * 0.5;

// ─── Colors (Jurassic Theme) ────────────────────────────────────────────────

const BG_COLOR: Color = Color::new(30, 26, 22, 255); // deep earth
const TILE_FACE: Color = Color::new(62, 54, 44, 255); // dark brown tile
const TILE_SIDE: Color = Color::new(45, 39, 32, 255); // darker side shadow
const TILE_BORDER: Color = Color::new(90, 78, 64, 255);
const TILE_HIGHLIGHT: Color = Color::new(255, 191, 0, 255); // neon amber
const HUD_TEXT: Color = Color::new(210, 190, 160, 255);
const VICTORY_COLOR: Color = Color::new(255, 200, 40, 255);

// ─── Tile Kinds ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TileKind {
    TRex,
    Raptor,
    Triceratops,
    Stegosaurus,
    Amber,
    Fossil,
    DNA,
    Fern,
}

impl TileKind {
    const ALL: [TileKind; 8] = [
        TileKind::TRex,
        TileKind::Raptor,
        TileKind::Triceratops,
        TileKind::Stegosaurus,
        TileKind::Amber,
        TileKind::Fossil,
        TileKind::DNA,
        TileKind::Fern,
    ];

    fn label(self) -> &'static str {
        match self {
            TileKind::TRex => "T-REX",
            TileKind::Raptor => "RAPTR",
            TileKind::Triceratops => "TRICE",
            TileKind::Stegosaurus => "STEGO",
            TileKind::Amber => "AMBER",
            TileKind::Fossil => "FOSIL",
            TileKind::DNA => " DNA ",
            TileKind::Fern => "FERN",
        }
    }

    fn icon_color(self) -> Color {
        match self {
            TileKind::TRex => Color::new(200, 60, 50, 255),
            TileKind::Raptor => Color::new(80, 180, 80, 255),
            TileKind::Triceratops => Color::new(100, 140, 200, 255),
            TileKind::Stegosaurus => Color::new(180, 130, 60, 255),
            TileKind::Amber => Color::new(255, 191, 0, 255),
            TileKind::Fossil => Color::new(170, 160, 150, 255),
            TileKind::DNA => Color::new(0, 200, 180, 255),
            TileKind::Fern => Color::new(60, 160, 60, 255),
        }
    }
}

// ─── Tile ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct Tile {
    kind: TileKind,
    gx: i32, // grid x (in half-tile units)
    gy: i32, // grid y (in half-tile units)
    gz: i32, // grid z (layer)
    removed: bool,
}

impl Tile {
    fn screen_x(&self, offset_x: f32, zoom: f32) -> f32 {
        offset_x + self.gx as f32 * GRID_UNIT_X * zoom - (self.gz as f32 * 3.0 * zoom)
    }

    fn screen_y(&self, offset_y: f32, zoom: f32) -> f32 {
        offset_y + self.gy as f32 * GRID_UNIT_Y * zoom - (self.gz as f32 * 3.0 * zoom)
    }

    fn screen_rect(&self, offset_x: f32, offset_y: f32, zoom: f32) -> Rectangle {
        Rectangle::new(
            self.screen_x(offset_x, zoom),
            self.screen_y(offset_y, zoom),
            TILE_W * zoom,
            TILE_H * zoom,
        )
    }
}

// ─── Game State ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Playing,
    Won,
}

pub mod layouts;
use layouts::Level;

struct Game {
    tiles: Vec<Tile>,
    selected: Option<usize>,
    matches_found: u32,
    total_pairs: u32,
    state: GameState,
    offset_x: f32,
    offset_y: f32,
    current_level: Level,
    zoom_factor: f32,
    pan_x: f32,
    pan_y: f32,
    level_scroll: usize,
}

impl Game {
    fn new() -> Self {
        let mut game = Game {
            tiles: Vec::new(),
            selected: None,
            matches_found: 0,
            total_pairs: 0,
            state: GameState::Playing,
            offset_x: 0.0,
            offset_y: 0.0,
            current_level: Level::Stegosaurus,
            zoom_factor: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
            level_scroll: 0,
        };
        game.generate_board();
        game
    }

    // ── Layouts ──────────────────────────────────────────────────────────

    fn generate_board(&mut self) {
        let mut rng = thread_rng();
        let positions = self.current_level.layout_positions();
        let num_tiles = positions.len();
        let num_pairs = num_tiles / 2;

        // Create pairs of tile kinds
        let mut kinds: Vec<TileKind> = Vec::with_capacity(num_tiles);
        for i in 0..num_pairs {
            let kind = TileKind::ALL[i % TileKind::ALL.len()];
            kinds.push(kind);
            kinds.push(kind);
        }
        kinds.shuffle(&mut rng);

        // Initialize board with dummy tiles.
        // Unassigned tiles are treated as physically present (removed = false).
        self.tiles.clear();
        for &(gx, gy, gz) in &positions {
            self.tiles.push(Tile {
                kind: TileKind::TRex, // dummy
                gx,
                gy,
                gz,
                removed: false,
            });
        }
        
        let mut unassigned_indices: Vec<usize> = (0..num_tiles).collect();
        let mut kind_index = 0;

        // Simulate game backwards: find free tiles, assign them a pair, and "remove" them
        while !unassigned_indices.is_empty() {
            let mut free_unassigned: Vec<usize> = Vec::new();
            for &idx in &unassigned_indices {
                if self.is_free(idx) {
                    free_unassigned.push(idx);
                }
            }

            // If we have less than 2 free tiles during reverse generation, we hit a rare topological deadlock.
            // Just restart the generation algorithm (Monte Carlo approach).
            if free_unassigned.len() < 2 {
                return self.generate_board(); 
            }

            // Pick 2 random free tiles
            free_unassigned.shuffle(&mut rng);
            let idx1 = free_unassigned[0];
            let idx2 = free_unassigned[1];

            // Assign the pair
            let kind = kinds[kind_index];
            self.tiles[idx1].kind = kind;
            self.tiles[idx2].kind = kind;
            kind_index += 2;

            // "Remove" them to unblock tiles underneath for the next iteration
            self.tiles[idx1].removed = true;
            self.tiles[idx2].removed = true;

            unassigned_indices.retain(|&i| i != idx1 && i != idx2);
        }

        // Successfully generated a guaranteed-solvable board!
        // Reset all tiles back to visible/not removed to start the game.
        for tile in &mut self.tiles {
            tile.removed = false;
        }

        self.selected = None;
        self.matches_found = 0;
        self.total_pairs = num_pairs as u32;
        self.state = GameState::Playing;

        // Center the layout on screen
        self.compute_offsets();
    }

    fn compute_offsets(&mut self) {
        let zoom = self.zoom_factor;
        let scaled_tile_w = TILE_W * zoom;
        let scaled_tile_h = TILE_H * zoom;

        let (mut min_x, mut max_x, mut min_y, mut max_y) =
            (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
        for t in &self.tiles {
            // Apply bounds calculation including the 3D depth shift!
            let sx = t.gx as f32 * GRID_UNIT_X * zoom - (t.gz as f32 * 3.0 * zoom);
            let sy = t.gy as f32 * GRID_UNIT_Y * zoom - (t.gz as f32 * 3.0 * zoom);
            if sx < min_x { min_x = sx; }
            if sx + scaled_tile_w > max_x { max_x = sx + scaled_tile_w; }
            if sy < min_y { min_y = sy; }
            if sy + scaled_tile_h > max_y { max_y = sy + scaled_tile_h; }
        }
        let board_w = max_x - min_x;
        let board_h = max_y - min_y;
        
        // Base screen centering
        self.offset_x = (SCREEN_W as f32 - board_w) / 2.0 - min_x;
        self.offset_y = (SCREEN_H as f32 - board_h) / 2.0 - min_y + 20.0;
        
        // Reset pan
        self.pan_x = 0.0;
        self.pan_y = 0.0;
    }

    // ── Freedom Check ────────────────────────────────────────────────────

    fn is_free(&self, idx: usize) -> bool {
        let tile = &self.tiles[idx];
        if tile.removed {
            return false;
        }

        // Check: no tile above (overlapping in x,y at z+1)
        for (i, other) in self.tiles.iter().enumerate() {
            if i == idx || other.removed {
                continue;
            }
            if other.gz == tile.gz + 1 {
                // Check overlap: tiles occupy 2 grid units wide, 2 tall
                let dx = (other.gx - tile.gx).abs();
                let dy = (other.gy - tile.gy).abs();
                if dx < 2 && dy < 2 {
                    return false;
                }
            }
        }

        // Check: left OR right side is open
        let left_blocked = self.tiles.iter().enumerate().any(|(i, other)| {
            i != idx
                && !other.removed
                && other.gz == tile.gz
                && other.gx == tile.gx - 2
                && (other.gy - tile.gy).abs() < 2
        });

        let right_blocked = self.tiles.iter().enumerate().any(|(i, other)| {
            i != idx
                && !other.removed
                && other.gz == tile.gz
                && other.gx == tile.gx + 2
                && (other.gy - tile.gy).abs() < 2
        });

        !left_blocked || !right_blocked
    }

    fn has_valid_moves(&self) -> bool {
        let free_tiles: Vec<usize> = (0..self.tiles.len())
            .filter(|&i| !self.tiles[i].removed && self.is_free(i))
            .collect();

        for i in 0..free_tiles.len() {
            for j in (i + 1)..free_tiles.len() {
                if self.tiles[free_tiles[i]].kind == self.tiles[free_tiles[j]].kind {
                    return true;
                }
            }
        }
        false
    }

    // ── Input Handling ───────────────────────────────────────────────────

    fn handle_input(&mut self, rl: &RaylibHandle) {
        if self.state == GameState::Won {
            // Proceed to next level or restart
            if rl.is_key_pressed(KeyboardKey::KEY_ENTER) || rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                if let Some(next_lvl) = self.current_level.next() {
                    self.current_level = next_lvl;
                } else {
                    // Start over campaign!
                    self.current_level = Level::Stegosaurus;
                }
                self.generate_board();
            } else if rl.is_key_pressed(KeyboardKey::KEY_R) {
                // Replay same level
                self.generate_board();
            }
            return;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            self.generate_board();
            return;
        }

        let mx = rl.get_mouse_x() as f32;
        let my = rl.get_mouse_y() as f32;

        // --- Panning ---
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let delta = rl.get_mouse_delta();
            self.pan_x += delta.x;
            self.pan_y += delta.y;
        }

        // --- Mouse Wheel Handling (Redirected if in sidebar) ---
        let wheel = rl.get_mouse_wheel_move();
        let levels = Level::ALL;
        let visible_count = 5; // We show 5 at a time

        if wheel != 0.0 {
            if mx < 80.0 {
                // Scroll the level list
                let mut new_scroll = self.level_scroll as f32 - wheel;
                if new_scroll < 0.0 { new_scroll = 0.0; }
                let max_scroll = levels.len().saturating_sub(visible_count) as f32;
                if new_scroll > max_scroll { new_scroll = max_scroll; }
                self.level_scroll = new_scroll as usize;
            } else {
                // --- Cursor-Centric Zooming ---
                // 1) Record mouse pos in board-space before zoom
                let board_mx_before = mx - self.offset_x - self.pan_x;
                let board_my_before = my - self.offset_y - self.pan_y;
                
                // 2) Apply zoom
                let old_zoom = self.zoom_factor;
                self.zoom_factor += wheel * 0.1;
                self.zoom_factor = self.zoom_factor.clamp(0.4, 3.5);
                
                // 3) Calculate new board space ratio changes
                let ratio = self.zoom_factor / old_zoom;
                let board_mx_after = board_mx_before * ratio;
                let board_my_after = board_my_before * ratio;
                
                // 4) Adjust pan to keep cursor stationary
                self.pan_x += board_mx_before - board_mx_after;
                self.pan_y += board_my_before - board_my_after;
            }
        }

        // Level selector interaction (Left edge)
        let mut clicked_level: Option<Level> = None;
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let spacing = 50.0;
            let visible_count = 5.min(levels.len());
            let arrow_h = 30.0;
            let total_h = visible_count as f32 * spacing + 2.0 * arrow_h;
            let start_y = (SCREEN_H as f32 - total_h) / 2.0;
            
            let up_arrow_rect = Rectangle::new(20.0, start_y, 40.0, arrow_h);
            let list_start_y = start_y + arrow_h;
            let down_arrow_rect = Rectangle::new(20.0, list_start_y + visible_count as f32 * spacing, 40.0, arrow_h);

            if mx >= up_arrow_rect.x && mx <= up_arrow_rect.x + up_arrow_rect.width && my >= up_arrow_rect.y && my <= up_arrow_rect.y + up_arrow_rect.height {
                if self.level_scroll >= 5 {
                    self.level_scroll -= 5;
                } else {
                    self.level_scroll = 0;
                }
            } else if mx >= down_arrow_rect.x && mx <= down_arrow_rect.x + down_arrow_rect.width && my >= down_arrow_rect.y && my <= down_arrow_rect.y + down_arrow_rect.height {
                if self.level_scroll + 5 < levels.len() {
                    self.level_scroll += 5;
                }
            } else {
                for i in 0..visible_count {
                    let lvl_idx = self.level_scroll + i;
                    if lvl_idx < levels.len() {
                        let rect = Rectangle::new(20.0, list_start_y + i as f32 * spacing, 40.0, 40.0);
                        if mx >= rect.x && mx <= rect.x + rect.width && my >= rect.y && my <= rect.y + rect.height {
                            clicked_level = Some(levels[lvl_idx]);
                            break;
                        }
                    }
                }
            }
        }

        if let Some(lvl) = clicked_level {
            if self.current_level != lvl {
                self.current_level = lvl;
                self.generate_board();
            }
            return;
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            // Test tiles from highest z to lowest (top tiles click first)
            let mut click_candidates: Vec<usize> = (0..self.tiles.len())
                .filter(|&i| !self.tiles[i].removed)
                .collect();
            click_candidates.sort_by(|&a, &b| {
                self.tiles[b]
                    .gz
                    .cmp(&self.tiles[a].gz)
                    .then(self.tiles[b].gy.cmp(&self.tiles[a].gy))
            });

            let zoom = self.zoom_factor;
            let _scaled_tile_w = TILE_W * zoom;
            let _scaled_tile_h = TILE_H * zoom;
            let scaled_depth = TILE_DEPTH * zoom;

            let mut clicked_idx: Option<usize> = None;
            for &idx in &click_candidates {
                let rect = self.tiles[idx].screen_rect(self.offset_x + self.pan_x, self.offset_y + self.pan_y, zoom);
                let gz = self.tiles[idx].gz as f32;
                let z_off = gz * scaled_depth;
                let shadow_d = scaled_depth + z_off * 0.3;

                // Expand click box to include the 3D rendered sides
                // Right side is 3.0 * zoom pixels wide, bottom side is shadow_d tall
                let side_width = 3.0 * zoom;
                if mx >= rect.x
                    && mx <= rect.x + rect.width + side_width
                    && my >= rect.y
                    && my <= rect.y + rect.height + shadow_d
                {
                    if self.is_free(idx) {
                        clicked_idx = Some(idx);
                        break;
                    }
                }
            }

            if let Some(idx) = clicked_idx {
                match self.selected {
                    None => {
                        self.selected = Some(idx);
                    }
                    Some(sel) => {
                        if sel == idx {
                            // Deselect
                            self.selected = None;
                        } else if self.tiles[sel].kind == self.tiles[idx].kind {
                            // Match!
                            self.tiles[sel].removed = true;
                            self.tiles[idx].removed = true;
                            self.selected = None;
                            self.matches_found += 1;

                            if self.matches_found == self.total_pairs {
                                self.state = GameState::Won;
                            }
                        } else {
                            // Different — select new tile
                            self.selected = Some(idx);
                        }
                    }
                }
            }
        }
    }

    // ── Rendering ────────────────────────────────────────────────────────

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(BG_COLOR);

        // Draw subtle background texture pattern
        self.draw_background(d);

        // Sort tiles for draw order: lowest z first, then by y
        let mut draw_order: Vec<usize> = (0..self.tiles.len())
            .filter(|&i| !self.tiles[i].removed)
            .collect();
        draw_order.sort_by(|&a, &b| {
            self.tiles[a]
                .gz
                .cmp(&self.tiles[b].gz)
                .then(self.tiles[a].gy.cmp(&self.tiles[b].gy))
                .then(self.tiles[a].gx.cmp(&self.tiles[b].gx))
        });

        for &idx in &draw_order {
            self.draw_tile(d, idx);
        }

        // HUD
        self.draw_hud(d);

        // Victory screen
        if self.state == GameState::Won {
            self.draw_victory(d);
        } else if !self.has_valid_moves() && self.state == GameState::Playing {
            self.draw_no_moves(d);
        }
    }

    fn draw_background(&self, d: &mut RaylibDrawHandle) {
        // Subtle grid dots for atmosphere
        let dot_spacing = 40;
        for x in (0..SCREEN_W).step_by(dot_spacing as usize) {
            for y in (0..SCREEN_H).step_by(dot_spacing as usize) {
                d.draw_pixel(x, y, Color::new(50, 44, 38, 80));
            }
        }

        // Title / branding
        d.draw_text("JURASSIC  MAHJONG", 20, 12, 28, Color::new(80, 70, 56, 180));
    }

    fn draw_tile(&self, d: &mut RaylibDrawHandle, idx: usize) {
        let zoom = self.zoom_factor;
        let scaled_w = TILE_W * zoom;
        let scaled_h = TILE_H * zoom;
        let scaled_depth = TILE_DEPTH * zoom;

        let tile = &self.tiles[idx];
        let sx = tile.screen_x(self.offset_x + self.pan_x, zoom);
        let sy = tile.screen_y(self.offset_y + self.pan_y, zoom);
        let z_off = tile.gz as f32 * scaled_depth;
        let is_selected = self.selected == Some(idx);
        let is_free = self.is_free(idx);

        // 3D side shadow (bottom-right offset)
        let shadow_d = scaled_depth + z_off * 0.3;
        d.draw_rectangle_rounded(
            Rectangle::new(sx + 2.0 * zoom, sy + shadow_d, scaled_w, scaled_h),
            0.12,
            4,
            Color::new(20, 18, 15, 200),
        );

        // Side faces (give 3D depth illusion)
        let side_width = 3.0 * zoom;
        // Right side
        let side_pts = [
            Vector2::new(sx + scaled_w, sy + scaled_h),
            Vector2::new(sx + scaled_w + side_width, sy + scaled_h + shadow_d),
            Vector2::new(sx + scaled_w + side_width, sy + shadow_d),
            Vector2::new(sx + scaled_w, sy),
        ];
        d.draw_triangle(side_pts[0], side_pts[1], side_pts[2], TILE_SIDE);
        d.draw_triangle(side_pts[0], side_pts[2], side_pts[3], TILE_SIDE);

        // Bottom side
        let bot_pts = [
            Vector2::new(sx, sy + scaled_h),
            Vector2::new(sx + side_width, sy + scaled_h + shadow_d),
            Vector2::new(sx + scaled_w + side_width, sy + scaled_h + shadow_d),
            Vector2::new(sx + scaled_w, sy + scaled_h),
        ];
        d.draw_triangle(bot_pts[0], bot_pts[1], bot_pts[2], TILE_SIDE);
        d.draw_triangle(bot_pts[0], bot_pts[2], bot_pts[3], TILE_SIDE);

        // Tile face
        let face_color = if is_selected {
            Color::new(100, 85, 55, 255)
        } else if is_free {
            TILE_FACE
        } else {
            Color::new(50, 44, 38, 255) // dimmed
        };

        d.draw_rectangle_rounded(
            Rectangle::new(sx, sy, scaled_w, scaled_h),
            0.12,
            4,
            face_color,
        );

        // Border
        let border_color = if is_selected {
            TILE_HIGHLIGHT
        } else {
            TILE_BORDER
        };

        d.draw_rectangle_rounded_lines(
            Rectangle::new(sx, sy, scaled_w, scaled_h),
            0.12,
            4,
            border_color,
        );

        // Glow effect for selected
        if is_selected {
            let glow = 2.0 * zoom;
            d.draw_rectangle_rounded_lines(
                Rectangle::new(sx - glow, sy - glow, scaled_w + glow * 2.0, scaled_h + glow * 2.0),
                0.12,
                4,
                Color::new(255, 191, 0, 120),
            );
        }

        // Icon / symbol
        self.draw_tile_icon(d, tile.kind, sx, sy, zoom);

        // Label text
        let label = tile.kind.label();
        let font_size = (10.0 * zoom).max(1.0) as i32;
        let text_w = d.measure_text(label, font_size);
        d.draw_text(
            label,
            (sx + (scaled_w - text_w as f32) / 2.0) as i32,
            (sy + scaled_h - (16.0 * zoom)) as i32,
            font_size,
            Color::new(180, 165, 140, 200),
        );
    }

    fn draw_tile_icon(&self, d: &mut RaylibDrawHandle, kind: TileKind, sx: f32, sy: f32, zoom: f32) {
        let cx = sx + (TILE_W * zoom) / 2.0;
        let cy = sy + (TILE_H * zoom) / 2.0 - (4.0 * zoom);
        let color = kind.icon_color();

        let s = |val: f32| val * zoom; // standard uniform scaler

        match kind {
            TileKind::TRex => {
                // Fierce head silhouette: triangle jaw + circle eye
                d.draw_triangle(
                    Vector2::new(cx - s(12.0), cy - s(10.0)),
                    Vector2::new(cx + s(14.0), cy),
                    Vector2::new(cx - s(12.0), cy + s(10.0)),
                    color,
                );
                d.draw_circle(cx as i32 - s(4.0) as i32, cy as i32 - s(4.0) as i32, s(3.0), BG_COLOR);
                // Teeth
                for i in 0..3 {
                    let tx = cx + s(2.0) + i as f32 * s(4.0);
                    d.draw_triangle(
                        Vector2::new(tx, cy + s(5.0)),
                        Vector2::new(tx + s(2.0), cy + s(10.0)),
                        Vector2::new(tx + s(4.0), cy + s(5.0)),
                        color,
                    );
                }
            }
            TileKind::Raptor => {
                // Sleek claw shape
                d.draw_triangle(
                    Vector2::new(cx, cy - s(14.0)),
                    Vector2::new(cx - s(10.0), cy + s(8.0)),
                    Vector2::new(cx + s(10.0), cy + s(8.0)),
                    color,
                );
                d.draw_triangle(
                    Vector2::new(cx + s(6.0), cy - s(4.0)),
                    Vector2::new(cx + s(16.0), cy + s(4.0)),
                    Vector2::new(cx + s(6.0), cy + s(4.0)),
                    color,
                );
                d.draw_circle(cx as i32 - s(2.0) as i32, cy as i32 - s(4.0) as i32, s(2.0), BG_COLOR);
            }
            TileKind::Triceratops => {
                // Shield / frill + horns
                d.draw_circle(cx as i32, cy as i32 + s(2.0) as i32, s(12.0), color);
                d.draw_circle(cx as i32, cy as i32 + s(2.0) as i32, s(8.0), Color::new(
                    color.r.saturating_sub(40),
                    color.g.saturating_sub(40),
                    color.b.saturating_sub(40),
                    255,
                ));
                // Horns
                d.draw_triangle(
                    Vector2::new(cx - s(6.0), cy - s(6.0)),
                    Vector2::new(cx - s(4.0), cy - s(16.0)),
                    Vector2::new(cx - s(2.0), cy - s(6.0)),
                    color,
                );
                d.draw_triangle(
                    Vector2::new(cx + s(2.0), cy - s(6.0)),
                    Vector2::new(cx + s(4.0), cy - s(16.0)),
                    Vector2::new(cx + s(6.0), cy - s(6.0)),
                    color,
                );
            }
            TileKind::Stegosaurus => {
                // Body + back plates
                d.draw_ellipse(cx as i32, cy as i32 + s(4.0) as i32, s(16.0), s(8.0), color);
                for i in 0..5 {
                    let px = cx - s(10.0) + i as f32 * s(5.0);
                    d.draw_triangle(
                        Vector2::new(px, cy - s(2.0)),
                        Vector2::new(px + s(2.5), cy - s(10.0)),
                        Vector2::new(px + s(5.0), cy - s(2.0)),
                        color,
                    );
                }
            }
            TileKind::Amber => {
                // Glowing gem
                d.draw_poly(Vector2::new(cx, cy), 6, s(14.0), 0.0, Color::new(255, 160, 0, 180));
                d.draw_poly(Vector2::new(cx, cy), 6, s(10.0), 30.0, color);
                d.draw_poly(Vector2::new(cx, cy), 6, s(5.0), 0.0, Color::new(255, 230, 150, 200));
            }
            TileKind::Fossil => {
                // Spiral ammonite
                d.draw_circle(cx as i32, cy as i32, s(12.0), color);
                d.draw_circle(cx as i32, cy as i32, s(9.0), TILE_FACE);
                d.draw_circle(cx as i32 + s(2.0) as i32, cy as i32 - s(1.0) as i32, s(7.0), color);
                d.draw_circle(cx as i32 + s(2.0) as i32, cy as i32 - s(1.0) as i32, s(5.0), TILE_FACE);
                d.draw_circle(cx as i32 + s(3.0) as i32, cy as i32 - s(2.0) as i32, s(3.0), color);
            }
            TileKind::DNA => {
                // Double helix hint
                for i in 0..7 {
                    let yy = cy - s(12.0) + i as f32 * s(4.0);
                    let off = if i % 2 == 0 { s(5.0) } else { -s(5.0) };
                    d.draw_circle((cx + off) as i32, yy as i32, s(2.5), color);
                    d.draw_circle((cx - off) as i32, yy as i32, s(2.5), color);
                    let line_thick = s(1.0).max(1.0);
                    d.draw_line_ex(
                        Vector2::new(cx + off, yy),
                        Vector2::new(cx - off, yy),
                        line_thick,
                        Color::new(color.r, color.g, color.b, 120),
                    );
                }
            }
            TileKind::Fern => {
                // Fern frond
                let line_thick = s(2.0).max(1.0);
                d.draw_line_ex(
                    Vector2::new(cx, cy + s(14.0)),
                    Vector2::new(cx, cy - s(14.0)),
                    line_thick,
                    color,
                );
                for i in 0..5 {
                    let yy = cy - s(10.0) + i as f32 * s(5.0);
                    let len = s(8.0 - i as f32 * 1.0);
                    d.draw_line_ex(
                        Vector2::new(cx, yy),
                        Vector2::new(cx - len, yy - s(3.0)),
                        s(1.5).max(1.0),
                        color,
                    );
                    d.draw_line_ex(
                        Vector2::new(cx, yy),
                        Vector2::new(cx + len, yy - s(3.0)),
                        s(1.5).max(1.0),
                        color,
                    );
                }
            }
        }
    }

    fn draw_hud(&self, d: &mut RaylibDrawHandle) {
        // Level Name — Top Center
        let name = self.current_level.name();
        let name_tw = d.measure_text(name, 24);
        d.draw_text(name, (SCREEN_W - name_tw) / 2, 16, 24, HUD_TEXT);

        // Level Selector (Left edge, vertical)
        let levels = Level::ALL;
        
        let spacing = 50.0;
        let visible_count = 5.min(levels.len());
        let arrow_h = 30.0;
        let total_h = visible_count as f32 * spacing + 2.0 * arrow_h;
        let start_y = (SCREEN_H as f32 - total_h) / 2.0;

        let list_start_y = start_y + arrow_h;
        let down_arrow_rect_y = list_start_y + visible_count as f32 * spacing;

        // Draw UP arrow
        if self.level_scroll > 0 {
            let cx = 40.0;
            let cy = start_y + 15.0;
            d.draw_triangle(
                Vector2::new(cx, cy - 8.0),
                Vector2::new(cx - 10.0, cy + 8.0),
                Vector2::new(cx + 10.0, cy + 8.0),
                Color::new(200, 180, 150, 255)
            );
        }

        // Draw visible levels
        for i in 0..visible_count {
            let lvl_idx = self.level_scroll + i;
            if lvl_idx >= levels.len() { break; }
            let lvl = levels[lvl_idx];
            let rect_y = list_start_y + i as f32 * spacing;
            let is_active = lvl == self.current_level;
            
            let color = if is_active {
                Color::new(255, 191, 0, 200) // Highlight amber
            } else {
                Color::new(80, 70, 56, 180) // Dim
            };

            d.draw_rectangle_rounded(
                Rectangle::new(20.0, rect_y, 40.0, 40.0),
                0.2,
                4,
                color,
            );
            
            // Number text inside
            let text = format!("{}", lvl_idx + 1);
            let tw = d.measure_text(&text, 20);
            d.draw_text(
                &text,
                (20.0 + (40.0 - tw as f32) / 2.0) as i32,
                (rect_y + 10.0) as i32,
                20,
                if is_active { BG_COLOR } else { HUD_TEXT },
            );
        }

        // Draw DOWN arrow
        if self.level_scroll + visible_count < levels.len() {
            let cx = 40.0;
            let cy = down_arrow_rect_y + 15.0;
            d.draw_triangle(
                Vector2::new(cx - 10.0, cy - 8.0),
                Vector2::new(cx, cy + 8.0),
                Vector2::new(cx + 10.0, cy - 8.0),
                Color::new(200, 180, 150, 255)
            );
        }

        // Matches counter — Top Right
        let text = format!("Matches: {} / {}", self.matches_found, self.total_pairs);
        let tw = d.measure_text(&text, 24);
        d.draw_text(&text, SCREEN_W - tw - 20, 16, 24, HUD_TEXT);

        // Controls hint
        d.draw_text(
            "[R] Restart",
            SCREEN_W / 2 - 50,
            SCREEN_H - 30,
            16,
            Color::new(120, 110, 95, 150),
        );

        // Free tiles count
        let free_count = (0..self.tiles.len())
            .filter(|&i| !self.tiles[i].removed && self.is_free(i))
            .count();
        d.draw_text(
            &format!("Free: {}", free_count),
            20,
            SCREEN_H - 30,
            16,
            Color::new(120, 110, 95, 150),
        );
    }

    fn draw_victory(&self, d: &mut RaylibDrawHandle) {
        // Semi-transparent overlay
        d.draw_rectangle(0, 0, SCREEN_W, SCREEN_H, Color::new(0, 0, 0, 180));

        // Victory text with amber glow
        let text = "VICTORY!";
        let font_size = 80;
        let tw = d.measure_text(text, font_size);
        let tx = (SCREEN_W - tw) / 2;
        let ty = SCREEN_H / 2 - 60;

        // Glow layers using positional offsets instead of font scaling
        // This avoids the "stretched" scaling artifacts of the default bitmap font
        let offsets = [
            (-2, -2), (0, -3), (2, -2),
            (-3, 0),           (3, 0),
            (-2, 2),  (0, 3),  (2, 2),
        ];

        // Draw multiple passes for a soft glow
        for pass in 1..=3 {
            let spread = pass * 2;
            let alpha = 80 / pass as u8;
            for &(ox, oy) in &offsets {
                d.draw_text(
                    text,
                    tx + ox * spread,
                    ty + oy * spread,
                    font_size,
                    Color::new(255, 191, 0, alpha),
                );
            }
        }

        d.draw_text(text, tx, ty, font_size, VICTORY_COLOR);

        let sub = if self.current_level.next().is_some() {
            "Click or press [Enter] for Next Level, [R] to Replay"
        } else {
            "Campaign Complete! Click to restart campaign."
        };
        let stw = d.measure_text(sub, 22);
        d.draw_text(
            sub,
            (SCREEN_W - stw) / 2,
            ty + 100,
            22,
            HUD_TEXT,
        );
    }

    fn draw_no_moves(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(0, 0, SCREEN_W, SCREEN_H, Color::new(0, 0, 0, 140));

        let text = "NO MOVES LEFT";
        let font_size = 50;
        let tw = d.measure_text(text, font_size);
        d.draw_text(
            text,
            (SCREEN_W - tw) / 2,
            SCREEN_H / 2 - 40,
            font_size,
            Color::new(200, 80, 60, 255),
        );

        let sub = "Press [R] to start a new game";
        let stw = d.measure_text(sub, 22);
        d.draw_text(
            sub,
            (SCREEN_W - stw) / 2,
            SCREEN_H / 2 + 30,
            22,
            HUD_TEXT,
        );
    }
}

// ─── Entry Point ─────────────────────────────────────────────────────────────

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_W, SCREEN_H)
        .title("Jurassic Mahjong Solitaire")
        .vsync()
        .build();

    rl.set_target_fps(60);

    let mut game = Game::new();

    while !rl.window_should_close() {
        // Update
        game.handle_input(&rl);

        // Draw
        let mut d = rl.begin_drawing(&thread);
        game.draw(&mut d);
    }
}
