use texture::Texture;


static CUBE_INDEXES : [uint, ..6] = [0, 1, 2, 0, 2, 3];

struct CubeSide {
    pos: [[f32, ..3], ..4],
    normal: [f32, ..3],
    tex_coord: [[f32, ..2], ..4],
}

static CUBE_SIDE_NEAR : CubeSide = CubeSide {
    pos: [
        [-1.0, -1.0,  1.0],
        [ 1.0, -1.0,  1.0],
        [ 1.0,  1.0,  1.0],
        [-1.0,  1.0,  1.0],
    ],
    normal: [0.0, 0.0, 1.0],
    tex_coord: [
        [0.0, 1.0],
        [1.0, 1.0],
        [1.0, 0.0],
        [0.0, 0.0],
    ]
};

static CUBE_SIDE_FAR : CubeSide = CubeSide {
    pos: [
        [-1.0, -1.0, -1.0],
        [-1.0,  1.0, -1.0],
        [ 1.0,  1.0, -1.0],
        [ 1.0, -1.0, -1.0],
    ],
    normal: [0.0, 0.0, -1.0],
    tex_coord: [
        [1.0, 1.0],
        [1.0, 0.0],
        [0.0, 0.0],
        [0.0, 1.0],
    ]
};

static CUBE_SIDE_TOP : CubeSide = CubeSide {
    pos: [
        [-1.0,  1.0, -1.0],
        [-1.0,  1.0,  1.0],
        [ 1.0,  1.0,  1.0],
        [ 1.0,  1.0, -1.0],
    ],
    normal: [0.0, 1.0, 0.0],
    tex_coord: [
        [1.0, 1.0],
        [1.0, 0.0],
        [0.0, 0.0],
        [0.0, 1.0],
    ]
};

static CUBE_SIDE_BOTTOM : CubeSide = CubeSide {
    pos: [
        [-1.0, -1.0, -1.0],
        [ 1.0, -1.0, -1.0],
        [ 1.0, -1.0,  1.0],
        [-1.0, -1.0,  1.0],
    ],
    normal: [0.0, -1.0, 0.0],
    tex_coord: [
        [0.0, 1.0],
        [1.0, 1.0],
        [1.0, 0.0],
        [0.0, 0.0],
    ]
};

static CUBE_SIDE_LEFT : CubeSide = CubeSide {
    pos: [
        [-1.0, -1.0, -1.0],
        [-1.0, -1.0,  1.0],
        [-1.0,  1.0,  1.0],
        [-1.0,  1.0, -1.0],
    ],
    normal: [-1.0, 0.0, 0.0],
    tex_coord: [
        [0.0, 1.0],
        [1.0, 1.0],
        [1.0, 0.0],
        [0.0, 0.0],
    ]
};

static CUBE_SIDE_RIGHT : CubeSide = CubeSide {
    pos: [
        [ 1.0, -1.0, -1.0],
        [ 1.0,  1.0, -1.0],
        [ 1.0,  1.0,  1.0],
        [ 1.0, -1.0,  1.0],
    ],
    normal: [1.0, 0.0, 0.0],
    tex_coord: [
        [1.0, 1.0],
        [1.0, 0.0],
        [0.0, 0.0],
        [0.0, 1.0],
    ]
};


#[vertex_format]
pub struct Vertex {
    #[name = "a_Pos"]
    pub pos: [f32, ..3],
    #[name = "a_TexCoord"]
    pub tex_coord: [f32, ..2],
    #[name = "a_Normal"]
    pub normal: [f32, ..3],
}

pub struct CubeMaker {
    vertices: Vec<Vertex>,
    indexes: Vec<u16>,
}

impl CubeMaker {
    pub fn new() -> CubeMaker {
        CubeMaker {
            vertices: vec![],
            indexes: vec![],
        }
    }

    pub fn add_all_sides(&mut self, pos: (f32, f32, f32), size: f32, tex: &Texture) {
        self.add_near_side(pos, size, tex);
        self.add_far_side(pos, size, tex);
        self.add_left_side(pos, size, tex);
        self.add_right_side(pos, size, tex);
        self.add_top_side(pos, size, tex);
        self.add_bottom_side(pos, size, tex);
    }

    pub fn add_near_side(&mut self, pos: (f32, f32, f32), size: f32,
                         tex: &Texture) {
        self.add_side(&CUBE_SIDE_NEAR, pos, size, tex)
    }

    pub fn add_far_side(&mut self, pos: (f32, f32, f32), size: f32,
                         tex: &Texture) {
        self.add_side(&CUBE_SIDE_FAR, pos, size, tex)
    }

    pub fn add_left_side(&mut self, pos: (f32, f32, f32), size: f32,
                         tex: &Texture) {
        self.add_side(&CUBE_SIDE_LEFT, pos, size, tex)
    }

    pub fn add_right_side(&mut self, pos: (f32, f32, f32), size: f32,
                          tex: &Texture) {
        self.add_side(&CUBE_SIDE_RIGHT, pos, size, tex)
    }

    pub fn add_top_side(&mut self, pos: (f32, f32, f32), size: f32,
                        tex: &Texture) {
        self.add_side(&CUBE_SIDE_TOP, pos, size, tex)
    }

    pub fn add_bottom_side(&mut self, pos: (f32, f32, f32), size: f32,
                           tex: &Texture) {
        self.add_side(&CUBE_SIDE_BOTTOM, pos, size, tex)
    }

    pub fn finish(self) -> (Vec<Vertex>, Vec<u16>) {
        (self.vertices, self.indexes)
    }

    fn add_side(&mut self, cs: &CubeSide, pos: (f32, f32, f32),
                size: f32, tex: &Texture) {
        let halfsize = size / 2.0;
        let (x, y, z) = pos;
        let (fac_x, fac_y, off_x, off_y) = tex.get_measurements();

        for &i in CUBE_INDEXES.iter() {
            let [cx, cy, cz] = cs.pos[i];
            let [tx, ty] = cs.tex_coord[i];
            self.indexes.push(self.vertices.len() as u16);
            self.vertices.push(Vertex {
                pos: [x + cx * halfsize, y + cy * halfsize, z + cz * halfsize],
                tex_coord: [tx * fac_x + off_x, ty * fac_y + off_y],
                normal: cs.normal,
            });
        }
    }
}
