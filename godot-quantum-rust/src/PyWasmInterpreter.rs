use godot::prelude::*;
use wasmtime::*;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

struct PyWasmInterpreter {
    engine: Engine,
    store: Store<WasiCtx>,
    module: Module,
    instance: Option<Instance>,
}

impl PyWasmInterpreter {
    fn new() -> Option<Self> {
        let engine = Engine::default();
        let wasm_bytes: &[u8] = include_bytes!("../pyodide/pyodide.asm.wasm");
        let module = Module::new(&engine, wasm_bytes);
        let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
        let mut store = Store::new(&engine, wasi_ctx);
        let result = match module{
            Ok(a) => Some(PyWasmInterpreter {
                engine,
                store,
                module:a,
                instance: None,
            }),
            Err(b) =>{ godot_print!("Failed to load Pyodide WASM module"); None},
        };
        result
    }

    fn instantiate(&mut self) {
        godot_print!("🔍 Instantiating Pyodide...");
    
        let mut linker = Linker::new(&self.engine);
        godot_print!("✅ Linker created.");    

        match linker.instantiate(&mut self.store, &self.module) {
            Ok(instance) => {
                godot_print!("✅ Pyodide WASM instantiated successfully.");
                self.instance = Some(instance);
            }
            Err(err) => godot_print!("❌ Failed to instantiate Pyodide: {:?}", err),
        }
    
        godot_print!("✅ Finished instantiate()");
    }
    
    fn run_python(&mut self, script: &str) -> String {
        if let Some(instance) = &self.instance {
            let run_func = instance.get_typed_func::<(i32, i32), i32>(&mut self.store, "run_python");
    
            match run_func {
                Ok(run_func) => {
                    let script_ptr = self.allocate_string(script);
                    let result_ptr = run_func.call(&mut self.store, (script_ptr, script.len() as i32));
    
                    match result_ptr {
                        Ok(ptr) => self.read_string(ptr),
                        Err(err) => format!("❌ Error executing Python: {:?}", err),
                    }
                }
                Err(err) => format!("❌ Failed to get Pyodide function: {:?}", err),
            }
        } else {
            "❌ Pyodide not initialized! Call instantiate() first.".to_string()
        }    
    }

    fn allocate_string(&mut self, text: &str) -> i32 {
        // Handle WASM memory allocation
        0 // Placeholder
    }

    fn read_string(&mut self, ptr: i32) -> String {
        // Read output from WASM memory
        "Python result".to_string() // Placeholder
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
struct PythonRunner {
    base: Base<Node>,
    py_wasm: Option<PyWasmInterpreter>,
}

#[godot_api]
impl INode for PythonRunner {
    fn init(base: Base<Node>) -> Self {
        godot_print!("🔍 Initializing PythonRunner...");

        let mut py_wasm = PyWasmInterpreter::new();
        if let Some(ref mut wasm) = py_wasm {
            godot_print!("✅ PyWasmInterpreter successfully created!");
            wasm.instantiate();
        } else {
            godot_print!("❌ Could not initialize PyWasmInterpreter!");
        }

        Self { base, py_wasm }
    }
}

#[godot_api]
impl PythonRunner {
    #[func]
    fn run_python_script(&mut self, script: GString) -> GString {
        let result = match &mut self.py_wasm{
            Some(a) => a.run_python(script.to_string().as_str()),
            None => "failed to run python".to_string(),
        };
        GString::from(result)
    }
}
