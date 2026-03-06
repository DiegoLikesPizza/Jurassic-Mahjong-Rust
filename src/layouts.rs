#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level {
    Stegosaurus,
    Brachiosaurus,
    Triceratops,
    TRex,
    Pterodactyl,
    Ankylosaurus,
    Velociraptor,
    Spinosaurus,
    Diplodocus,
    Parasaurolophus,
    Iguanodon,
    Carnotaurus,
    Allosaurus,
    Pachycephalosaurus,
    Therizinosaurus,
    Dimetrodon,
    Mosasaurus,
    Plesiosaurus,
    Compsognathus,
    Gallimimus,
}

impl Level {
    pub const ALL: [Level; 20] = [
        Level::Stegosaurus,
        Level::Brachiosaurus,
        Level::Triceratops,
        Level::TRex,
        Level::Pterodactyl,
        Level::Ankylosaurus,
        Level::Velociraptor,
        Level::Spinosaurus,
        Level::Diplodocus,
        Level::Parasaurolophus,
        Level::Iguanodon,
        Level::Carnotaurus,
        Level::Allosaurus,
        Level::Pachycephalosaurus,
        Level::Therizinosaurus,
        Level::Dimetrodon,
        Level::Mosasaurus,
        Level::Plesiosaurus,
        Level::Compsognathus,
        Level::Gallimimus,
    ];

    pub fn next(&self) -> Option<Self> {
        match self {
            Level::Stegosaurus => Some(Level::Brachiosaurus),
            Level::Brachiosaurus => Some(Level::Triceratops),
            Level::Triceratops => Some(Level::TRex),
            Level::TRex => Some(Level::Pterodactyl),
            Level::Pterodactyl => Some(Level::Ankylosaurus),
            Level::Ankylosaurus => Some(Level::Velociraptor),
            Level::Velociraptor => Some(Level::Spinosaurus),
            Level::Spinosaurus => Some(Level::Diplodocus),
            Level::Diplodocus => Some(Level::Parasaurolophus),
            Level::Parasaurolophus => Some(Level::Iguanodon),
            Level::Iguanodon => Some(Level::Carnotaurus),
            Level::Carnotaurus => Some(Level::Allosaurus),
            Level::Allosaurus => Some(Level::Pachycephalosaurus),
            Level::Pachycephalosaurus => Some(Level::Therizinosaurus),
            Level::Therizinosaurus => Some(Level::Dimetrodon),
            Level::Dimetrodon => Some(Level::Mosasaurus),
            Level::Mosasaurus => Some(Level::Plesiosaurus),
            Level::Plesiosaurus => Some(Level::Compsognathus),
            Level::Compsognathus => Some(Level::Gallimimus),
            Level::Gallimimus => None, // Game beaten!
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Level::Stegosaurus => "Level 1: Stegosaurus",
            Level::Brachiosaurus => "Level 2: Brachiosaurus",
            Level::Triceratops => "Level 3: Triceratops",
            Level::TRex => "Level 4: T-Rex",
            Level::Pterodactyl => "Level 5: Pterodactyl",
            Level::Ankylosaurus => "Level 6: Ankylosaurus",
            Level::Velociraptor => "Level 7: Velociraptor",
            Level::Spinosaurus => "Level 8: Spinosaurus",
            Level::Diplodocus => "Level 9: Diplodocus",
            Level::Parasaurolophus => "Level 10: Parasaurolophus",
            Level::Iguanodon => "Level 11: Iguanodon",
            Level::Carnotaurus => "Level 12: Carnotaurus",
            Level::Allosaurus => "Level 13: Allosaurus",
            Level::Pachycephalosaurus => "Level 14: Pachycephalosaurus",
            Level::Therizinosaurus => "Level 15: Therizinosaurus",
            Level::Dimetrodon => "Level 16: Dimetrodon",
            Level::Mosasaurus => "Level 17: Mosasaurus",
            Level::Plesiosaurus => "Level 18: Plesiosaurus",
            Level::Compsognathus => "Level 19: Compsognathus",
            Level::Gallimimus => "Level 20: Gallimimus",
        }
    }
}

impl Level {
    pub fn layout_positions(&self) -> Vec<(i32, i32, i32)> {
        match self {
            Level::Stegosaurus => Self::layout_stegosaurus(),
            Level::Brachiosaurus => Self::layout_brachiosaurus(),
            Level::Triceratops => Self::layout_triceratops(),
            Level::TRex => Self::layout_trex(),
            Level::Pterodactyl => Self::layout_pterodactyl(),
            Level::Ankylosaurus => Self::layout_ankylosaurus(),
            Level::Velociraptor => Self::layout_velociraptor(),
            Level::Spinosaurus => Self::layout_spinosaurus(),
            Level::Diplodocus => Self::layout_diplodocus(),
            Level::Parasaurolophus => Self::layout_parasaurolophus(),
            Level::Iguanodon => Self::layout_iguanodon(),
            Level::Carnotaurus => Self::layout_carnotaurus(),
            Level::Allosaurus => Self::layout_allosaurus(),
            Level::Pachycephalosaurus => Self::layout_pachycephalosaurus(),
            Level::Therizinosaurus => Self::layout_therizinosaurus(),
            Level::Dimetrodon => Self::layout_dimetrodon(),
            Level::Mosasaurus => Self::layout_mosasaurus(),
            Level::Plesiosaurus => Self::layout_plesiosaurus(),
            Level::Compsognathus => Self::layout_compsognathus(),
            Level::Gallimimus => Self::layout_gallimimus(),
        }
    }

    // ── Level 5: Pterodactyl (Flying, wide wings) ──
    pub fn layout_pterodactyl() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();

        // LAYER 0 (88 tiles)
        let layer0 = [
            (-12, vec![(-2, 2)]), // Head tip (beak)
            (-10, vec![(-2, 3)]), // Beak
            (-8, vec![(-4, 5)]),  // Head & crest
            (-6, vec![(-6, 7)]),  // Neck
            (-4, vec![(-20, 2), (-10, 11), (14, 2)]), // Wing tips + body
            (-2, vec![(-16, 4), (-6, 7), (10, 4)]),   // Wings spreading
            (0, vec![(-12, 14)]), // Massive wingspan
            (2, vec![(-12, 14)]),
            (4, vec![(-8, 5), (-2, 2), (4, 5)]), // Wings trailing edge
            (6, vec![(-6, 2), (2, 2)]), // Legs trailing
            (8, vec![(-4, 1), (2, 1)]), // Feet
        ];

        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }

        // LAYER 1 (40 tiles)
        let layer1 = [
            (-8, vec![(-2, 3)]), // Head center
            (-6, vec![(-4, 4)]), // Neck
            (-4, vec![(-8, 9)]), // Inner wings
            (-2, vec![(-8, 9)]),
            (0, vec![(-6, 7)]),
            (2, vec![(-4, 5)]),
            (4, vec![(-2, 3)]), // Tail/body end
        ];

        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }

        // LAYER 2 (16 tiles)
        let layer2 = [
            (-6, vec![(-2, 2)]),
            (-4, vec![(-4, 5)]), // Wing joints / shoulders
            (-2, vec![(-2, 3)]), // Center back
            (0, vec![(-2, 3)]),
            (2, vec![(0, 1)]),
        ];

        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }

        if positions.len() % 2 != 0 {
            positions.pop();
        }
        positions
    }

    // ── Level 6: Ankylosaurus (Armored, club tail) ──
    pub fn layout_ankylosaurus() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();

        // LAYER 0 (76 tiles)
        let layer0 = [
            (-6, vec![(-10, 3)]), // Head
            (-4, vec![(-12, 5)]), // Neck
            (-2, vec![(-14, 10)]), // Wide armored body
            (0, vec![(-16, 12)]),
            (2, vec![(-14, 10)]),
            (4, vec![(-12, 8)]), // Back of body
            (6, vec![(-10, 2), (-4, 2)]), // Legs
            (2, vec![(6, 3)]), // Tail start
            (4, vec![(8, 3)]),
            (6, vec![(10, 4)]), // Tail club
        ];

        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }

        // LAYER 1 (50 tiles)
        let layer1 = [
            (-2, vec![(-12, 8)]), // Main shell
            (0, vec![(-14, 10)]),
            (2, vec![(-12, 8)]),
            (4, vec![(-10, 5)]),
            (2, vec![(6, 2)]), // Tail armor
            (4, vec![(8, 2)]),
            (6, vec![(12, 2)]), // Club center
        ];

        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }

        // LAYER 2 (18 tiles)
        let layer2 = [
            (-2, vec![(-8, 5)]), // Spikes/ridges
            (0, vec![(-10, 7)]),
            (2, vec![(-8, 5)]),
            (6, vec![(12, 1)]), // Club tip
        ];

        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }

        if positions.len() % 2 != 0 {
            positions.pop();
        }
        positions
    }

    // ── Level 7: Velociraptor (Agile, running, long stiff tail) ──
    pub fn layout_velociraptor() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();

        // LAYER 0 (80 tiles)
        let layer0 = [
            (-10, vec![(-18, 3)]), // Snout
            (-8, vec![(-20, 5)]),  // Head & crest
            (-6, vec![(-16, 4)]),  // Neck
            (-4, vec![(-14, 4)]),  // Upper body
            (-2, vec![(-12, 5)]),  // Mid body
            (0, vec![(-10, 5), (-14, 2)]), // Lower body, tiny arms
            (2, vec![(-8, 5)]),    // Hips
            (4, vec![(-6, 4), (-10, 2)]), // Thighs, legs
            (6, vec![(-4, 4), (-8, 2)]), // Tail start, feet
            (8, vec![(-2, 5)]),    // Tail mid
            (10, vec![(8, 6)]),    // Tail end (stiff, straight)
        ];

        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }

        // LAYER 1 (46 tiles)
        let layer1 = [
            (-8, vec![(-18, 4)]), // Head core
            (-6, vec![(-14, 3)]), // Neck core
            (-4, vec![(-12, 3)]), // Body core
            (-2, vec![(-10, 4)]),
            (0, vec![(-8, 4)]), // Core body
            (2, vec![(-6, 4)]), // Hips core
            (4, vec![(-4, 3)]), // Tail base
            (6, vec![(-2, 4)]), // Tail
            (8, vec![(6, 4)]),  // Tail
        ];

        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }

        // LAYER 2 (18 tiles)
        let layer2 = [
            (-8, vec![(-16, 2)]), // Eye ridge
            (-4, vec![(-10, 2)]), // Shoulder
            (-2, vec![(-8, 3)]),  // Spine
            (0, vec![(-6, 3)]),   // Spine
            (4, vec![(-2, 2)]),   // Hip joint
        ];

        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }

        if positions.len() % 2 != 0 {
            positions.pop();
        }
        positions
    }

    // ── Level 8: Spinosaurus (Large sail on back, long snout) ──
    pub fn layout_spinosaurus() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();

        // LAYER 0 (78 tiles)
        let layer0 = [
            (-8, vec![(-24, 4)]), // Long snout
            (-6, vec![(-26, 6)]), // Head
            (-4, vec![(-20, 5)]), // Neck
            (-2, vec![(-16, 12)]), // Huge body & sail base
            (0, vec![(-14, 12)]),
            (2, vec![(-12, 10)]),
            (4, vec![(-10, 2), (-4, 3)]), // Arm, leg
            (6, vec![(-10, 2), (-2, 2)]), // Claws, feet
            (2, vec![(8, 4)]),  // Tail 1
            (4, vec![(16, 4)]), // Tail 2
            (6, vec![(24, 3)]), // Tail 3
        ];

        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }

        // LAYER 1 (50 tiles)
        let layer1 = [
            (-6, vec![(-24, 4)]), // Head core
            (-4, vec![(-18, 4)]), // Neck core
            (-2, vec![(-14, 10)]), // Core body
            (0, vec![(-12, 10)]),
            (2, vec![(-10, 8)]),
            (4, vec![(6, 2)]), // Tail core
            (6, vec![(10, 2)]),
        ];

        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }

        // LAYER 2 (16 tiles) - The Sail
        let layer2 = [
            (-8, vec![(-10, 4)]), // Top of sail
            (-6, vec![(-12, 6)]), // Mid sail
            (-4, vec![(-14, 8)]), // Base of sail
        ];

        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }

        if positions.len() % 2 != 0 {
            positions.pop();
        }
        positions
    }

    // ── Level 9: Diplodocus (Extremely long neck & tail) ──
    pub fn layout_diplodocus() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();

        // LAYER 0 (76 tiles)
        let layer0 = [
            (-12, vec![(-30, 2)]), // Small head
            (-10, vec![(-28, 2)]), // Neck curving down
            (-8, vec![(-26, 2)]),
            (-6, vec![(-24, 2)]),
            (-4, vec![(-22, 2)]),
            (-2, vec![(-20, 2), (-12, 6)]), // Neck base + front body
            (0, vec![(-18, 2), (-14, 8)]),  // Neck + main body
            (2, vec![(-16, 12)]),           // Huge body
            (4, vec![(-14, 10), (-6, 2), (2, 2)]), // Body + front legs
            (6, vec![(-12, 8), (-4, 2), (4, 2)]),  // Lower body + feet
            (0, vec![(4, 4)]),  // Tail start
            (2, vec![(8, 4)]),  // Tail
            (4, vec![(16, 4)]), // Tail extending
            (6, vec![(24, 4)]), // Whip tail end
        ];

        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }

        // LAYER 1 (50 tiles)
        let layer1 = [
            (-8, vec![(-28, 2)]), // Neck core
            (-6, vec![(-26, 2)]),
            (-4, vec![(-24, 2)]),
            (-2, vec![(-22, 2), (-10, 5)]), // Neck core + body core
            (0, vec![(-20, 2), (-12, 7)]),
            (2, vec![(-14, 9)]),  // Main body core
            (4, vec![(-10, 5)]),  // Lower body core
            (0, vec![(4, 3)]),  // Tail core
            (2, vec![(8, 3)]),  // Tail core
            (4, vec![(16, 3)]), // Tail extending
        ];

        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }

        // LAYER 2 (18 tiles)
        let layer2 = [
            (-2, vec![(-8, 3)]), // Shoulders
            (0, vec![(-10, 4)]), // Back
            (2, vec![(-12, 5)]), // Hips / Spine
            (0, vec![(4, 2)]), // Tail base
        ];

        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }

        if positions.len() % 2 != 0 {
            positions.pop();
        }
        positions
    }

    // ── Level 10: Parasaurolophus (Long backward crest, bipedal stance) ──
    pub fn layout_parasaurolophus() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();

        // LAYER 0 (78 tiles)
        let layer0 = [
            (-12, vec![(-18, 5)]), // Beak & Crest tip
            (-10, vec![(-16, 6)]), // Head & Crest mid
            (-8, vec![(-14, 5)]),  // Head base
            (-6, vec![(-12, 4)]),  // Neck
            (-4, vec![(-10, 5)]),  // Upper body
            (-2, vec![(-8, 6)]),   // Mid body
            (0, vec![(-8, 7), (-14, 2)]), // Lower body, arms
            (2, vec![(-6, 7)]),    // Hips
            (4, vec![(-4, 5), (-8, 2)]), // Thighs, legs
            (6, vec![(-2, 4), (-6, 2)]), // Feet
            (8, vec![(0, 2)]),     // Toes
            (2, vec![(8, 4)]),   // Tail base
            (4, vec![(12, 4)]),  // Tail mid
            (6, vec![(18, 4)]),  // Tail end
        ];

        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }

        // LAYER 1 (46 tiles)
        let layer1 = [
            (-10, vec![(-14, 4)]), // Crest core
            (-8, vec![(-12, 3)]),  // Head core
            (-6, vec![(-10, 3)]),  // Neck core
            (-4, vec![(-8, 4)]),   // Body core upper
            (-2, vec![(-6, 5)]),   // Body core mid
            (0, vec![(-6, 5)]),    // Body core lower
            (2, vec![(-4, 4)]),    // Hips core
            (4, vec![(-2, 3)]),    // Thigh core
            (2, vec![(8, 3)]),   // Tail core base
            (4, vec![(12, 3)]),  // Tail core mid
        ];

        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }

        // LAYER 2 (20 tiles)
        let layer2 = [
            (-8, vec![(-10, 2)]), // Eye area
            (-4, vec![(-6, 2)]),  // Shoulder
            (-2, vec![(-4, 3)]),  // Spine mid
            (0, vec![(-4, 3)]),   // Spine lower
            (2, vec![(-2, 3)]),   // Hip joint
            (4, vec![(0, 2)]),    // Tail joint
        ];

        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }

        if positions.len() % 2 != 0 {
            positions.pop();
        }
        positions
    }

    pub fn layout_brachiosaurus() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();

        // ── LAYER 0 (95 tiles) ──
        let l0_gy_minus18 = [(-14, 2)];
        let l0_gy_minus16 = [(-14, 3)];
        let l0_gy_minus14 = [(-12, 3)];
        let l0_gy_minus12 = [(-10, 3)];
        let l0_gy_minus10 = [(-8, 3)];
        let l0_gy_minus8 = [(-6, 6)];
        let l0_gy_minus6 = [(-4, 8)];
        let l0_gy_minus4 = [(-8, 1), (-4, 9)];
        let l0_gy_minus2 = [(-12, 1), (-4, 10)];
        let l0_gy_0 = [(-16, 1), (-4, 10)];
        let l0_gy_2 = [(-20, 3), (-4, 10)];
        let l0_gy_4 = [(-2, 8)];
        let l0_gy_6 = [(0, 6)];
        let l0_gy_8 = [(-2, 2), (8, 2)];
        let l0_gy_10 = [(-2, 2), (8, 2)];

        let layer0 = [
            (-18, &l0_gy_minus18[..]),
            (-16, &l0_gy_minus16[..]),
            (-14, &l0_gy_minus14[..]),
            (-12, &l0_gy_minus12[..]),
            (-10, &l0_gy_minus10[..]),
            (-8, &l0_gy_minus8[..]),
            (-6, &l0_gy_minus6[..]),
            (-4, &l0_gy_minus4[..]),
            (-2, &l0_gy_minus2[..]),
            (0, &l0_gy_0[..]),
            (2, &l0_gy_2[..]),
            (4, &l0_gy_4[..]),
            (6, &l0_gy_6[..]),
            (8, &l0_gy_8[..]),
            (10, &l0_gy_10[..]),
        ];

        for &(gy, segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }

        // ── LAYER 1 (41 tiles) ──
        let l1_gy_minus8 = [(-4, 3)];
        let l1_gy_minus6 = [(-2, 5)];
        let l1_gy_minus4 = [(-2, 6)];
        let l1_gy_minus2 = [(-2, 7)];
        let l1_gy_0 = [(-2, 7)];
        let l1_gy_2 = [(-2, 7)];
        let l1_gy_4 = [(0, 5)];
        let l1_gy_6 = [(4, 1)];

        let layer1 = [
            (-8, &l1_gy_minus8[..]),
            (-6, &l1_gy_minus6[..]),
            (-4, &l1_gy_minus4[..]),
            (-2, &l1_gy_minus2[..]),
            (0, &l1_gy_0[..]),
            (2, &l1_gy_2[..]),
            (4, &l1_gy_4[..]),
            (6, &l1_gy_6[..]),
        ];

        for &(gy, segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }

        // ── LAYER 2 (8 tiles) ──
        let l2_gy_minus6 = [(0, 3)];
        let l2_gy_minus4 = [(0, 3)];
        let l2_gy_minus2 = [(2, 2)];

        let layer2 = [
            (-6, &l2_gy_minus6[..]),
            (-4, &l2_gy_minus4[..]),
            (-2, &l2_gy_minus2[..]),
        ];

        for &(gy, segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }

        if positions.len() % 2 != 0 {
            positions.pop();
        }

        positions
    }

    pub fn layout_stegosaurus() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();

        // ── LAYER 0 (82 tiles) ──
        // Spikes / Plates Peak (3 tiles)
        let l0_gy_minus6 = [(-8, 1), (0, 1), (8, 1)];
        // Spikes Base (6 tiles, half-tile offset!)
        let l0_gy_minus4 = [(-9, 2), (-1, 2), (7, 2)];
        // Back (9 tiles)
        let l0_gy_minus2 = [(-8, 9)];
        // Upper Body (12 tiles)
        let l0_gy_0 = [(-14, 12)];
        // Mid Body (15 tiles)
        let l0_gy_2 = [(-18, 15)];
        // Tail + Body + Head (18 tiles)
        let l0_gy_4 = [(-26, 18)];
        // Lower Body (13 tiles)
        let l0_gy_6 = [(-14, 13)];
        // Legs Top (4 tiles)
        let l0_gy_8 = [(-10, 2), (4, 2)];
        // Feet (2 tiles)
        let l0_gy_10 = [(-10, 1), (6, 1)];

        let layer0 = [
            (-6, &l0_gy_minus6[..]),
            (-4, &l0_gy_minus4[..]),
            (-2, &l0_gy_minus2[..]),
            (0, &l0_gy_0[..]),
            (2, &l0_gy_2[..]),
            (4, &l0_gy_4[..]),
            (6, &l0_gy_6[..]),
            (8, &l0_gy_8[..]),
            (10, &l0_gy_10[..]),
        ];

        for &(gy, segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }

        // ── LAYER 1 (54 tiles) ──
        let l1_gy_minus4 = [(-8, 1), (0, 1), (8, 1)];
        let l1_gy_minus2 = [(-6, 7)];
        let l1_gy_0 = [(-10, 10)];
        let l1_gy_2 = [(-14, 11)];
        let l1_gy_4 = [(-16, 12)];
        let l1_gy_6 = [(-10, 9)];
        let l1_gy_8 = [(-8, 1), (4, 1)];

        let layer1 = [
            (-4, &l1_gy_minus4[..]),
            (-2, &l1_gy_minus2[..]),
            (0, &l1_gy_0[..]),
            (2, &l1_gy_2[..]),
            (4, &l1_gy_4[..]),
            (6, &l1_gy_6[..]),
            (8, &l1_gy_8[..]),
        ];

        for &(gy, segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }

        // ── LAYER 2 (8 tiles) ──
        let l2_gy_0 = [(-2, 2)];
        let l2_gy_2 = [(-4, 4)];
        let l2_gy_4 = [(-2, 2)];

        let layer2 = [
            (0, &l2_gy_0[..]),
            (2, &l2_gy_2[..]),
            (4, &l2_gy_4[..]),
        ];

        for &(gy, segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }

        // Ensure exactly pairs
        if positions.len() % 2 != 0 {
            positions.pop();
        }

        positions
    }

    pub fn layout_triceratops() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();

        // ── LAYER 0 (78 tiles) ──
        let l0_gy_minus10 = [(6, 2)];
        let l0_gy_minus8 = [(4, 4)];
        let l0_gy_minus6 = [(2, 7)];
        let l0_gy_minus4 = [(-4, 9)];
        let l0_gy_minus2 = [(-10, 12)];
        let l0_gy_0 = [(-14, 16)];
        let l0_gy_2 = [(-18, 15)];
        let l0_gy_4 = [(-12, 11)];
        let l0_gy_6 = [(-8, 1), (4, 1)];

        let layer0 = [
            (-10, &l0_gy_minus10[..]),
            (-8,  &l0_gy_minus8[..]),
            (-6,  &l0_gy_minus6[..]),
            (-4,  &l0_gy_minus4[..]),
            (-2,  &l0_gy_minus2[..]),
            (0,   &l0_gy_0[..]),
            (2,   &l0_gy_2[..]),
            (4,   &l0_gy_4[..]),
            (6,   &l0_gy_6[..]),
        ];

        for &(gy, segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }

        // ── LAYER 1 (48 tiles) ──
        let l1_gy_minus8 = [(6, 2)];
        let l1_gy_minus6 = [(4, 4)];
        let l1_gy_minus4 = [(-2, 7)];
        let l1_gy_minus2 = [(-8, 10)];
        let l1_gy_0 = [(-10, 10)];
        let l1_gy_2 = [(-10, 9)];
        let l1_gy_4 = [(-6, 6)];

        let layer1 = [
            (-8, &l1_gy_minus8[..]),
            (-6, &l1_gy_minus6[..]),
            (-4, &l1_gy_minus4[..]),
            (-2, &l1_gy_minus2[..]),
            (0,  &l1_gy_0[..]),
            (2,  &l1_gy_2[..]),
            (4,  &l1_gy_4[..]),
        ];

        for &(gy, segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }

        // ── LAYER 2 (18 tiles) ──
        let l2_gy_minus6 = [(6, 2)];
        let l2_gy_minus4 = [(4, 3)];
        let l2_gy_minus2 = [(-4, 6)];
        let l2_gy_0 = [(-6, 7)];

        let layer2 = [
            (-6, &l2_gy_minus6[..]),
            (-4, &l2_gy_minus4[..]),
            (-2, &l2_gy_minus2[..]),
            (0,  &l2_gy_0[..]),
        ];

        for &(gy, segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }

        if positions.len() % 2 != 0 {
            positions.pop();
        }

        positions
    }

    pub fn layout_trex() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();

        // ── LAYER 0 (90 tiles) ──
        let l0_gy_minus14 = [(10, 4)];
        let l0_gy_minus12 = [(6, 7)];
        let l0_gy_minus10 = [(6, 3)];
        let l0_gy_minus8 = [(4, 5)];
        let l0_gy_minus6 = [(0, 6)];
        let l0_gy_minus4 = [(-4, 7), (10, 1)]; // Arm!
        let l0_gy_minus2 = [(-10, 9)];
        let l0_gy_0 = [(-16, 11)];
        let l0_gy_2 = [(-20, 12)];
        let l0_gy_4 = [(-24, 13)];
        let l0_gy_6 = [(-26, 3), (-6, 1), (0, 1)];
        let l0_gy_8 = [(-6, 1), (0, 1)];
        let l0_gy_10 = [(-6, 1), (0, 1)];

        let layer0 = [
            (-14, &l0_gy_minus14[..]),
            (-12, &l0_gy_minus12[..]),
            (-10, &l0_gy_minus10[..]),
            (-8,  &l0_gy_minus8[..]),
            (-6,  &l0_gy_minus6[..]),
            (-4,  &l0_gy_minus4[..]),
            (-2,  &l0_gy_minus2[..]),
            (0,   &l0_gy_0[..]),
            (2,   &l0_gy_2[..]),
            (4,   &l0_gy_4[..]),
            (6,   &l0_gy_6[..]),
            (8,   &l0_gy_8[..]),
            (10,  &l0_gy_10[..]),
        ];

        for &(gy, segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }

        // ── LAYER 1 (44 tiles) ──
        let l1_gy_minus12 = [(8, 3)];
        let l1_gy_minus10 = [(6, 3)];
        let l1_gy_minus8 = [(4, 4)];
        let l1_gy_minus6 = [(2, 4)];
        let l1_gy_minus4 = [(-2, 5)];
        let l1_gy_minus2 = [(-6, 6)];
        let l1_gy_0 = [(-10, 7)];
        let l1_gy_2 = [(-14, 8)];
        let l1_gy_4 = [(-20, 4)];

        let layer1 = [
            (-12, &l1_gy_minus12[..]),
            (-10, &l1_gy_minus10[..]),
            (-8,  &l1_gy_minus8[..]),
            (-6,  &l1_gy_minus6[..]),
            (-4,  &l1_gy_minus4[..]),
            (-2,  &l1_gy_minus2[..]),
            (0,   &l1_gy_0[..]),
            (2,   &l1_gy_2[..]),
            (4,   &l1_gy_4[..]),
        ];

        for &(gy, segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }

        // ── LAYER 2 (10 tiles) ──
        let l2_gy_minus12 = [(10, 2)];
        let l2_gy_minus2 = [(-4, 3)];
        let l2_gy_0 = [(-4, 3)];
        let l2_gy_2 = [(-4, 2)];

        let layer2 = [
            (-12, &l2_gy_minus12[..]),
            (-2,  &l2_gy_minus2[..]),
            (0,   &l2_gy_0[..]),
            (2,   &l2_gy_2[..]),
        ];

        for &(gy, segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }

        if positions.len() % 2 != 0 {
            positions.pop();
        }

        positions
    }

    // ── Level 11: Iguanodon (Bulky quadruped/biped, distinctive thumb spike) ──
    pub fn layout_iguanodon() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();
        // LAYER 0 (84 tiles)
        let layer0 = [
            (-10, vec![(-16, 2)]), // Distinct head
            (-8, vec![(-18, 4)]),  // Neck
            (-6, vec![(-14, 6)]),  // Upper body
            (-4, vec![(-12, 10)]), // Mid body
            (-2, vec![(-10, 12)]), // Deep body
            (0, vec![(-10, 12)]),
            (2, vec![(-8, 10)]),  // Hips
            (4, vec![(-10, 2), (-4, 2), (2, 2)]), // Legs + tail start
            (6, vec![(-10, 1), (-4, 1), (6, 2)]), // Feet + tail
            (8, vec![(10, 4)]),    // Tail end
            (-4, vec![(-16, 1)]),  // Thumb spike left
            (-4, vec![(-1, 1)]),   // Thumb spike right
        ];
        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }
        // LAYER 1 (50 tiles)
        let layer1 = [
            (-8, vec![(-16, 3)]),
            (-6, vec![(-12, 5)]),
            (-4, vec![(-10, 8)]),
            (-2, vec![(-8, 9)]),
            (0, vec![(-8, 9)]),
            (2, vec![(-6, 7)]),
            (4, vec![(4, 3)]), // Tail core
        ];
        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }
        // LAYER 2 (16 tiles)
        let layer2 = [
            (-4, vec![(-6, 4)]), // Spine
            (-2, vec![(-4, 5)]),
            (0, vec![(-4, 5)]),
            (2, vec![(-2, 2)]),
        ];
        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }
        if positions.len() % 2 != 0 { positions.pop(); }
        positions
    }

    // ── Level 12: Carnotaurus (Bull-like horns, extremely short arms) ──
    pub fn layout_carnotaurus() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();
        // LAYER 0 (82 tiles)
        let layer0 = [
            (-12, vec![(-14, 3)]), // Short deep head
            (-10, vec![(-16, 5)]), // Head & horns (logic)
            (-8, vec![(-12, 4)]),  // Strong neck
            (-6, vec![(-10, 5)]),  // Body
            (-4, vec![(-8, 7)]),   // Bulky core
            (-2, vec![(-6, 8)]),
            (0, vec![(-6, 8)]),
            (2, vec![(-4, 6)]),    // Hips
            (4, vec![(-6, 2), (-2, 2)]), // Legs
            (6, vec![(-6, 1), (-2, 1), (4, 4)]), // Feet + Tail
            (8, vec![(10, 5)]),    // Long tail
        ];
        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }
        // LAYER 1 (48 tiles)
        let layer1 = [
            (-10, vec![(-14, 4)]), // Head core
            (-8, vec![(-12, 3)]),  // Neck
            (-6, vec![(-10, 5)]),  // Body core
            (-4, vec![(-8, 6)]),
            (-2, vec![(-6, 7)]),
            (0, vec![(-6, 7)]),
            (4, vec![(4, 2)]), // Tail base
            (6, vec![(8, 2)]), // Tail mid
        ];
        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }
        // LAYER 2 (14 tiles)
        let layer2 = [
            (-11, vec![(-15, 1), (-10, 1)]), // Horns!
            (-4, vec![(-4, 4)]), // Shoulder
            (-2, vec![(-2, 4)]), // Back
            (0, vec![(-2, 4)]),
        ];
        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }
        if positions.len() % 2 != 0 { positions.pop(); }
        positions
    }

    // ── Level 13: Allosaurus (Large predator, prominent brow ridges) ──
    pub fn layout_allosaurus() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();
        // LAYER 0 (88 tiles)
        let layer0 = [
            (-12, vec![(-20, 4)]), // Snout
            (-10, vec![(-22, 6)]), // Mid head
            (-8, vec![(-16, 5)]),  // Neck
            (-6, vec![(-12, 6)]),  // Body front
            (-4, vec![(-10, 8)]),  // Body mid
            (-2, vec![(-8, 9)]),
            (0, vec![(-10, 10)]),  // Deep belly
            (2, vec![(-8, 6)]),    // Hips
            (4, vec![(-8, 2), (-2, 2), (4, 4)]), // Legs + Tail start
            (6, vec![(-8, 1), (-2, 1), (10, 5)]), // Feet + Tail
            (8, vec![(18, 4)]),    // Tail end
        ];
        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }
        // LAYER 1 (44 tiles)
        let layer1 = [
            (-10, vec![(-20, 5)]), // Head core
            (-8, vec![(-14, 4)]),  // Neck
            (-6, vec![(-10, 5)]),  // Body depth
            (-4, vec![(-8, 7)]),
            (-2, vec![(-6, 7)]),
            (0, vec![(-6, 7)]),
            (4, vec![(6, 3)]), // Tail core
            (6, vec![(14, 1)]),
        ];
        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }
        // LAYER 2 (16 tiles)
        let layer2 = [
            (-11, vec![(-18, 1), (-14, 1)]), // Eye ridges!
            (-4, vec![(-4, 4)]), // Spine
            (-2, vec![(-2, 5)]),
            (0, vec![(-2, 5)]),
        ];
        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }
        if positions.len() % 2 != 0 { positions.pop(); }
        positions
    }

    // ── Level 14: Pachycephalosaurus (Thick bone dome on head) ──
    pub fn layout_pachycephalosaurus() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();
        // LAYER 0 (86 tiles)
        let layer0 = [
            (-10, vec![(-14, 4)]), // Large head
            (-8, vec![(-16, 6)]),  // Head & Dome base
            (-6, vec![(-12, 5)]),  // Neck
            (-4, vec![(-10, 7)]),  // Upper body
            (-2, vec![(-8, 9)]),   // Deep body
            (0, vec![(-6, 10)]),
            (2, vec![(-6, 8)]),    // Hips
            (4, vec![(-8, 2), (-2, 2)]), // Legs
            (6, vec![(-8, 1), (-2, 1), (4, 4)]), // Feet + Tail
            (8, vec![(10, 5)]),    // Stiff tail
        ];
        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }
        // LAYER 1 (46 tiles)
        let layer1 = [
            (-10, vec![(-12, 3)]), // Dome core
            (-8, vec![(-14, 4)]),  // Head core
            (-6, vec![(-10, 3)]),  // Neck
            (-4, vec![(-8, 5)]),   // Body
            (-2, vec![(-6, 6)]),
            (0, vec![(-6, 6)]),
            (4, vec![(4, 3)]), // Tail base
        ];
        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }
        // LAYER 2 (18 tiles)
        let layer2 = [
            (-11, vec![(-12, 2)]), // THE DOME top level
            (-9, vec![(-13, 3)]),  // Dome side bulk
            (-4, vec![(-4, 4)]),   // Spine
        ];
        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }
        if positions.len() % 2 != 0 { positions.pop(); }
        positions
    }

    // ── Level 15: Therizinosaurus (Giant scythe-like claws) ──
    pub fn layout_therizinosaurus() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();
        // LAYER 0 (92 tiles)
        let layer0 = [
            (-12, vec![(-2, 2)]),  // Small head
            (-10, vec![(-4, 3)]),  // Neck
            (-8, vec![(-6, 4)]),
            (-6, vec![(-8, 6)]),   // Body start
            (-4, vec![(-28, 3), (-10, 10), (12, 3)]), // CLAWS (left), body, CLAWS (right)
            (-2, vec![(-24, 2), (-12, 12), (18, 2)]), // Claws extending
            (0, vec![(-14, 14)]),  // Fat belly
            (2, vec![(-12, 12)]),
            (4, vec![(-10, 6)]),   // Hips
            (6, vec![(-10, 2), (-2, 2)]), // Legs
            (8, vec![(-10, 1), (-2, 1)]), // Feet
        ];
        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }
        // LAYER 1 (42 tiles)
        let layer1 = [
            (-10, vec![(-2, 2)]), // Neck core
            (-8, vec![(-4, 3)]),
            (-6, vec![(-6, 5)]),  // Body depth
            (-4, vec![(-8, 7)]),
            (-2, vec![(-10, 8)]),
            (0, vec![(-10, 8)]),
        ];
        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }
        // LAYER 2 (16 tiles)
        let layer2 = [
            (-4, vec![(-26, 1), (14, 1)]), // Scythe tips
            (-2, vec![(-4, 5)]), // Back ridge
            (0, vec![(-4, 5)]),
        ];
        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }
        if positions.len() % 2 != 0 { positions.pop(); }
        positions
    }

    // ── Level 16: Dimetrodon (Massive sail-backed synapsid) ──
    pub fn layout_dimetrodon() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();
        // LAYER 0 (84 tiles)
        let layer0 = [
            (-6, vec![(-20, 5)]), // Large head
            (-4, vec![(-22, 12)]), // Neck & body
            (-2, vec![(-18, 14)]), // Deep body
            (0, vec![(-16, 16)]),
            (2, vec![(-14, 14)]),
            (4, vec![(-20, 2), (-10, 2), (-2, 2), (6, 2)]), // Sprawled legs
            (6, vec![(-20, 1), (-10, 1), (-2, 1), (6, 1), (12, 6)]), // Feet + long tail
        ];
        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }
        // LAYER 1 (44 tiles)
        let layer1 = [
            (-4, vec![(-18, 8)]), // Body core
            (-2, vec![(-14, 10)]),
            (0, vec![(-12, 11)]),
            (2, vec![(-10, 9)]),
            (6, vec![(14, 4)]), // Tail core
        ];
        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }
        // LAYER 2 (24 tiles) - THE SAIL
        let layer2 = [
            (-12, vec![(-4, 2)]), // Sail top
            (-10, vec![(-6, 4)]),
            (-8, vec![(-8, 6)]),
            (-6, vec![(-10, 8)]), // Sail base
            (-4, vec![(-4, 4)]), // Anchor
        ];
        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }
        if positions.len() % 2 != 0 { positions.pop(); }
        positions
    }

    // ── Level 17: Mosasaurus (Marine, streamlined body, flippers) ──
    pub fn layout_mosasaurus() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();
        // LAYER 0 (84 tiles)
        let layer0 = [
            (-6, vec![(-18, 5)]), // Long jaw
            (-4, vec![(-20, 7)]), // Head
            (-2, vec![(-16, 5), (-26, 3), (2, 3)]), // Body + front flippers
            (0, vec![(-14, 12)]), // Deep body
            (2, vec![(-12, 10), (-4, 2), (6, 2)]), // Body + hind flippers
            (4, vec![(8, 6)]),    // Tail base
            (6, vec![(20, 4)]),   // Tail fluke
        ];
        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }
        // LAYER 1 (46 tiles)
        let layer1 = [
            (-4, vec![(-18, 5)]), // Head core
            (-2, vec![(-14, 8)]), // Body core
            (0, vec![(-12, 10)]),
            (2, vec![(-10, 8)]),
            (4, vec![(8, 2)]),  // Tail core
        ];
        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }
        // LAYER 2 (16 tiles)
        let layer2 = [
            (-2, vec![(-10, 5)]), // Spine
            (0, vec![(-8, 6)]),
            (2, vec![(-6, 5)]),
        ];
        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }
        if positions.len() % 2 != 0 { positions.pop(); }
        positions
    }

    // ── Level 18: Plesiosaurus (Marine, very long neck, 4 flippers) ──
    pub fn layout_plesiosaurus() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();
        // LAYER 0 (88 tiles)
        let layer0 = [
            (-12, vec![(-26, 2)]), // Small head
            (-10, vec![(-24, 2)]), // Long neck
            (-8, vec![(-22, 2)]),
            (-6, vec![(-20, 2)]),
            (-4, vec![(-18, 2), (-12, 8), (-24, 3), (-4, 3)]), // Body + front flippers
            (-2, vec![(-14, 10)]), // Bulky body
            (0, vec![(-12, 12)]),
            (2, vec![(-10, 10), (0, 3)]), // Body + hind flippers
            (4, vec![(8, 4)]),    // Short tail
        ];
        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }
        // LAYER 1 (42 tiles)
        let layer1 = [
            (-8, vec![(-24, 2)]), // Neck core
            (-6, vec![(-22, 2)]),
            (-4, vec![(-20, 2), (-10, 6)]), // Body core
            (-2, vec![(-12, 8)]),
            (0, vec![(-12, 8)]),
            (2, vec![(-10, 6)]),
        ];
        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }
        // LAYER 2 (14 tiles)
        let layer2 = [
            (-2, vec![(-8, 5)]), // Hump
            (0, vec![(-10, 6)]),
            (2, vec![(-4, 3)]),
        ];
        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }
        if positions.len() % 2 != 0 { positions.pop(); }
        positions
    }

    // ── Level 19: Compsognathus (Tiny, agile, long tail) ──
    pub fn layout_compsognathus() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();
        // LAYER 0 (82 tiles)
        let layer0 = [
            (-8, vec![(-6, 2)]),  // Tiny head
            (-6, vec![(-4, 2)]),  // Slim neck
            (-4, vec![(-2, 3)]),  // Body
            (-2, vec![(0, 4)]),
            (0, vec![(0, 3)]),
            (2, vec![(-2, 2)]),   // Tiny legs
            (4, vec![(-4, 2)]),   // Feet
            (-2, vec![(8, 10)]),  // VERY long skinny tail
            (0, vec![(28, 4)]),   // Tail end
        ];
        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }
        // LAYER 1 (46 tiles)
        let layer1 = [
            (-4, vec![(-2, 3)]), // Body depth
            (-2, vec![(0, 4)]),
            (0, vec![(0, 3)]),
            (-2, vec![(8, 12)]), // Tail core
        ];
        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }
        // LAYER 2 (16 tiles)
        let layer2 = [
            (-2, vec![(10, 8)]), // Spine / tail ridge
        ];
        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }
        if positions.len() % 2 != 0 { positions.pop(); }
        positions
    }

    // ── Level 20: Gallimimus (Ostrich-like, long neck & legs) ──
    pub fn layout_gallimimus() -> Vec<(i32, i32, i32)> {
        let mut positions = Vec::new();
        // LAYER 0 (88 tiles)
        let layer0 = [
            (-14, vec![(-4, 2)]), // Small head
            (-12, vec![(-4, 2)]), // Long neck
            (-10, vec![(-4, 2)]),
            (-8, vec![(-4, 2)]),
            (-6, vec![(-6, 3)]),  // Neck base
            (-4, vec![(-8, 6)]),  // Body core
            (-2, vec![(-10, 8)]),
            (0, vec![(-10, 8)]),
            (2, vec![(-8, 6)]),   // Hips
            (4, vec![(-8, 1), (-1, 1), (4, 4)]), // High legs + Tail
            (6, vec![(-8, 1), (-1, 1), (12, 5)]), // Long feet + Tail
            (8, vec![(-8, 1), (-1, 1)]), // Toes
        ];
        for &(gy, ref segments) in &layer0 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 0));
                }
            }
        }
        // LAYER 1 (48 tiles)
        let layer1 = [
            (-12, vec![(-4, 1)]), // Neck core
            (-10, vec![(-4, 1)]),
            (-8, vec![(-4, 1)]),
            (-6, vec![(-4, 2)]),
            (-4, vec![(-6, 4)]), // Body core
            (-2, vec![(-8, 6)]),
            (0, vec![(-8, 6)]),
            (4, vec![(4, 3)]), // Tail base
            (6, vec![(10, 2)]),
        ];
        for &(gy, ref segments) in &layer1 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 1));
                }
            }
        }
        // LAYER 2 (20 tiles)
        let layer2 = [
            (-2, vec![(-4, 4)]), // Spine
            (0, vec![(-4, 4)]),
            (2, vec![(-2, 2)]),
        ];
        for &(gy, ref segments) in &layer2 {
            for &(gx_start, count) in segments {
                for i in 0..count {
                    positions.push((gx_start + i * 2, gy, 2));
                }
            }
        }
        if positions.len() % 2 != 0 { positions.pop(); }
        positions
    }
}
