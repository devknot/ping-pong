#![feature(duration_constants)]

use coffee::graphics::{
	Color, Frame, Mesh, Rectangle, Shape, Window, WindowSettings, Point,
};
use coffee::input::{Keyboard, keyboard::KeyCode};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

use rand::{rngs::StdRng, Rng, SeedableRng};

const TAM_X: f32 = 640.0;
const TAM_Y: f32 = 480.0;

const CASA: u32 = 16;

const CASA_X: f32 = ((TAM_X as u32) / CASA) as f32;
const CASA_Y: f32 = ((TAM_Y as u32) / CASA) as f32;

const TAM_REBATEDOR: u32 = CASA/4;

fn main() -> Result<()> {
	Motor::run(WindowSettings {
		title: String::from("ping pong"),
		size: (TAM_X as u32, TAM_Y as u32),
		resizable: false,
		fullscreen: false,
		maximized: false,
	})
}

pub struct Motor {
	retangulos: [Rectangle<f32>; 2],
	bola: Rectangle<f32>,
	velocidade: Point,
}

impl Motor {
	pub fn gerar(theta: f32) -> Self{
		Self{
			retangulos: [
			    Rectangle{
			        x: CASA_X,
			        y: CASA_Y *((CASA/2)-TAM_REBATEDOR) as f32,
			        width: CASA_X,
			        height: CASA_Y * TAM_REBATEDOR as f32,
			    },
			    Rectangle{
			        x: CASA_X * (CASA-2) as f32,
			        y: CASA_Y *((CASA/2)-TAM_REBATEDOR) as f32,
			        width: CASA_X,
			        height: CASA_Y * TAM_REBATEDOR as f32,
			    },
			],
			bola: Rectangle {
                x: CASA_X * (CASA/2) as f32,
                y: CASA_Y * (CASA/2) as f32,
		        width: CASA_X/2.0,
			    height: CASA_Y/2.0,
			},
			velocidade: Point::new(
		        CASA_X/2.0 * theta.cos(),
		        CASA_Y/2.0 * theta.sin(),
		    ),
		}
	}
	
	fn bola(&self) -> Shape {
	    Shape::Circle {
            center: self.bola.center(),
            radius: (self.bola.width+self.bola.height)/2.0,
        }
	}
	
	fn verificar_bola_saiu_tela(&mut self) {
	    let c = self.bola.center();
	    let r = (self.bola.width+self.bola.height)/2.0;
	    
	    if (c[1] - r ) < r { // c[1] + r  < 0.0 {
	        self.velocidade[1] *= - 1.0;
	    }else if TAM_Y < c[1] + r {
	        self.velocidade[1] *= - 1.0;
	    }else if (c[0] - r ) < r { 
	        self.velocidade[0] *= - 1.0;
	    }else if TAM_X < c[0] + r {
	        self.velocidade[0] *= - 1.0;
	    }
	}
	
	fn verificar_bola_taco(&mut self, rec: Rectangle<f32>) {
	    if  self.bola.x < rec.x + rec.width        &&
	        self.bola.x + self.bola.width > rec.x  &&
	        self.bola.y + self.bola.height > rec.y &&
	        self.bola.y < rec.y + rec.height
	    {
	        self.velocidade[0] *= - 1.0;
	    }
	}
	fn verificar_lugar(&mut self) {
	    let c = self.bola.center();
	    let r = (self.bola.width+self.bola.height)/2.0;
	    
	    if c[0] - r < CASA_X  { 
	        //self.velocidade[0] *= - 1.0;
	        self.reniciar_bola();
	    }else if  c[0] + r >  (CASA_X * (CASA-2) as f32 + CASA_X/2.0) {
	        //self.velocidade[0] *= - 1.0;
	        self.reniciar_bola();
	    }
	}
	
	fn reniciar_bola(&mut self) {
	    let mut rng = StdRng::from_entropy();
		let theta: f32 = rng.gen_range(29.0..35.0);
		
	    self.velocidade = Point::new(
		    CASA_X/3.0 * theta.cos(),
		    CASA_Y/3.0 * theta.sin(),
		);
		
		self.retangulos = [
			    Rectangle{
			        x: CASA_X,
			        y: CASA_Y *((CASA/2)-TAM_REBATEDOR) as f32,
			        width: CASA_X,
			        height: CASA_Y * TAM_REBATEDOR as f32,
			    },
			    Rectangle{
			        x: CASA_X * (CASA-2) as f32,
			        y: CASA_Y *((CASA/2)-TAM_REBATEDOR) as f32,
			        width: CASA_X,
			        height: CASA_Y * TAM_REBATEDOR as f32,
			    },
			];
		
		self.bola.x = CASA_X * (CASA/2) as f32;
		self.bola.y = CASA_Y * (CASA/2) as f32;
	}
}

impl Game for Motor {
	type Input = Keyboard;
	type LoadingScreen = ();
	
	const TICKS_PER_SECOND: u16 = 30;
	const DEBUG_KEY: Option<KeyCode> = None;
	
	fn load(_window: &Window) -> Task<Motor> {
		Task::succeed(|| {
		    let mut rng = StdRng::from_entropy();
		    Motor::gerar( rng.gen_range(29.0..35.0)) 
		})
	}
	
	fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
		frame.clear(Color::BLACK);
		
		let mut mesh = Mesh::new();
		
		for rec in self.retangulos.iter() {
		    mesh.fill(Shape::Rectangle(*rec), Color::GREEN);
		}
		
		mesh.fill(self.bola(), Color::RED);
		
		mesh.draw(&mut frame.as_target());
		
	}
	
	fn interact(&mut self, input: &mut Self::Input, _window: &mut Window) {
		if input.was_key_released(KeyCode::Up) {
			if self.retangulos[0].y > 0.0 {
				self.retangulos[0].y -= CASA_Y;
			}
		}else if input.was_key_released(KeyCode::Down) {
			if  (self.retangulos[0].y + self.retangulos[0].width) <
			    (CASA_Y * (CASA-3) as f32)
			{
				self.retangulos[0].y += CASA_Y;
			}
		}
		
	}
	
	fn update(&mut self, _window: &Window) {
	    //let x = self.bola.x + self.velocidade[0];
	    //let y = self.bola.y + self.velocidade[1];
	    
	    self.bola.x += self.velocidade[0];
	    self.bola.y += self.velocidade[1];
	    
	    for rec in  0..2 {
	        self.verificar_bola_taco( self.retangulos[rec] );
	    }
	    
	    self.verificar_lugar();
	    
	    self.verificar_bola_saiu_tela();
	    
	}
	
	
}

