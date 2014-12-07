use time;
use cgmath;
use cgmath::FixedArray;
use cgmath::Matrix;
use gfx::{Device, DeviceHelper};
use glfw::{Context, WindowEvent};
use gfx;
use glfw;

use std::io::timer::sleep;
use std::time::duration::Duration;
use std::error::Error;

use errors::Res;
use engine::Engine;
use resources::ResourceLoader;
use texture::{Texture, BasicTexture};

#[shader_param(CubeBatch)]
struct Params {
    #[name = "u_Transform"]
    transform: [[f32, ..4], ..4],

    #[name = "u_Time"]
    time: f32,

    #[name = "t_Color"]
    color: gfx::shade::TextureParam,
}

static VERTEX_SRC: gfx::ShaderSource<'static> = shaders! {
GLSL_150: b"
    #version 150 core

    in vec3 a_Pos;
    in vec3 a_Normal;
    in vec2 a_TexCoord;
    out vec2 v_TexCoord;
    out vec3 v_Normal;
    out vec3 v_SunDirection;
    out vec3 v_HalfVec;

    uniform mat4 u_Transform;

    void main() {
        v_TexCoord = a_TexCoord;
        v_Normal = a_Normal;
        v_SunDirection = normalize(vec3(0.4, 0.3, 1.0));
        v_HalfVec = normalize(a_Pos + v_SunDirection);
        gl_Position = u_Transform * vec4(a_Pos, 1.0);
    }
"
};

static FRAGMENT_SRC: gfx::ShaderSource<'static> = shaders! {
GLSL_150: b"
    #version 150 core

    in vec2 v_TexCoord;
    in vec3 v_Normal;
    in vec3 v_HalfVec;
    in vec3 v_SunDirection;
    out vec4 o_Color;

    uniform sampler2D t_Color;

    void directionalLight(in vec3 normal,
                          in vec3 lightDir,
                          in vec3 halfVec,
                          in float shininess,
                          in vec4 lightDiffuse,
                          in vec4 lightSpecular,
                          inout vec4 diffuse,
                          inout vec4 specular)
    {
        float nDotVp; /* normal . light dir */
        float nDotHv; /* normal . half vec */
        float pf; /* power factor */
        nDotVp = max(0.0, dot(normal, normalize(lightDir)));
        nDotHv = max(0.0, dot(normal, halfVec));
        pf = (nDotVp == 0.0) ? 0.0 : pow(nDotHv, shininess);
        diffuse += lightDiffuse * nDotVp;
        specular += lightSpecular * pf;
    }

    void main() {
        vec4 darkness = vec4(0.1, 0.1, 0.1, 1.0);
        vec4 ambient = vec4(0.4, 0.4, 0.4, 1.0);
        vec4 sunColor = vec4(1.0, 1.0, 1.0, 0.3);
        vec4 diffuse = vec4(0.0);
        vec4 specular = vec4(0.0);

        directionalLight(v_Normal, v_SunDirection, v_HalfVec,
                         30.0, sunColor, vec4(0.0), diffuse,
                         specular);
                         
        vec4 color = texture(t_Color, v_TexCoord);
        color = color * clamp(darkness + ambient + diffuse, 0.0, 1.0);

        o_Color = color;
    }
"
};


fn run_everything() -> Res<()> {
    let engine = try!(Engine::new());
    let rl = ResourceLoader::new();

    let frame = engine.new_frame();
    let mut device = engine.new_device();

    let image = try!(rl.load_image("board.png"));
    let texture_map = try!(BasicTexture::from_image(&mut device, &image));
    let sampler = device.create_sampler(
        gfx::tex::SamplerInfo::new(gfx::tex::FilterMethod::Bilinear,
                                   gfx::tex::WrapMode::Clamp)
    );

    let map = try!(rl.load_map("map001.json"));
    let map_mesh = map.create_mesh(&mut device, &texture_map);

    let program = try!(device.link_program(VERTEX_SRC.clone(), FRAGMENT_SRC.clone()));
    let state = gfx::DrawState::new().depth(gfx::state::Comparison::LessEqual, true);

    let mut graphics = gfx::Graphics::new(device);
    let batch: CubeBatch = try!(graphics.make_batch(
        &program, map_mesh.get_mesh(), map_mesh.get_slice(), &state));

    let view = map.get_camera_view();
    let proj = cgmath::perspective(cgmath::deg(30.0f32),
        engine.get_framebuffer_aspect(), 0.1, 100.0);

    let mut data = Params {
        transform: proj.mul_m(&view.mat).into_fixed(),
        time: 0.0,
        color: (texture_map.handle(), Some(sampler)),
    };

    let clear_data = gfx::ClearData {
        color: [0.3, 0.3, 0.3, 1.0],
        depth: 1.0,
        stencil: 0,
    };

    let started = time::precise_time_s();

    while !engine.window.should_close() {
        engine.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&engine.events) {
            match event {
                WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) =>
                    engine.window.set_should_close(true),
                _ => {},
            }
        }

        data.time = (time::precise_time_s() - started) as f32;

        graphics.clear(clear_data, gfx::COLOR | gfx::DEPTH, &frame);
        graphics.draw(&batch, &data, &frame);
        graphics.end_frame();

        engine.window.swap_buffers();
        sleep(Duration::milliseconds(13));
    }

    Ok(())
}

pub fn run() {
    if let Err(err) = run_everything() {
        println!("Error: Something went wrong!");
        println!("  {}", err.description());
        match err.detail() {
            Some(detail) => { println!("  detail: {}", detail); }
            None => {}
        }
    }
}
