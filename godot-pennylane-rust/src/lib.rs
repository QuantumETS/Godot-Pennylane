use godot::prelude::*;
use godot::engine::Sprite2D;
use godot::engine::ISprite2D;
// use pyo3::prelude::*;
// use pyo3::types::IntoPyDict;

mod circuit;
struct PennylaneGodotRust;

#[gdextension]
unsafe impl ExtensionLibrary for PennylaneGodotRust {}



#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }
    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be: 
        // rotation += angular_speed * delta
        
        let radians = (self.angular_speed * delta * self.speed) as f32;
        self.base_mut().rotate(radians);
        // The 'rotate' method requires a f32, 
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
        let rotation = self.base().get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        //self.base_mut().translate(velocity * delta as f32);
        
        // or verbose: 
        // let this = self.base_mut();
        // this.set_position(
        //     this.position() + velocity * delta as f32
        // );
    }
}

#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
        self.base_mut().emit_signal("speed_increased".into(), &[]);
    }
    #[func]
    fn get_speed(&mut self) -> f64 {
        self.speed
    }
    // #[func]
    // fn get_test_string() -> String{
    //     Self::execute_test_python().unwrap()
    // }
    // fn execute_test_python() -> PyResult<String>{
    //     Python::with_gil(|py| {
    //         let sys = py.import_bound("sys")?;
    //         let version: String = sys.getattr("version")?.extract()?;
            
    //         let locals = [("os", py.import_bound("os")?)].into_py_dict_bound(py);
    //         let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
    //         let user: String = py.eval_bound(code, None, Some(&locals))?.extract()?;
    //         println!("Hello {}, I'm Python {}", user, version);
    //         Ok(user)
    //     })
    // }
    #[signal]
    fn speed_increased();
}