use godot::prelude::*;
use wasmtime::*;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};
use std::fs::read_to_string;

struct PyWasmInterpreter {
    engine: Engine,
    store: Store<WasiCtx>,
    module: Module,
    instance: Option<Instance>,
}

impl PyWasmInterpreter {
    fn new() -> Self {
        let engine = Engine::default();
        let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
        let mut store = Store::new(&engine, wasi_ctx);

        let module = Module::from_file(&engine, "godot-quantum-rust\\pyodide\\pyodide.asm.wasm")
            .expect("Failed to load Pyodide WASM module");

        PyWasmInterpreter {
            engine,
            store,
            module,
            instance: None,
        }
    }

    fn instantiate(&mut self) {
        let linker = Linker::new(&self.engine);
        let instance = linker.instantiate(&mut self.store, &self.module)
            .expect("Failed to instantiate Pyodide");

        self.instance = Some(instance);
    }

    fn run_python(&mut self, script: &str) -> String {
        if let Some(instance) = &self.instance {
            let run_func = instance.get_typed_func::<(i32, i32), i32>(&mut self.store, "run_python")
                .expect("Failed to get Pyodide function");

            let script_ptr = self.allocate_string(script);
            let result_ptr = run_func.call(&mut self.store, (script_ptr, script.len() as i32))
                .expect("Failed to execute Python");

            self.read_string(result_ptr)
        } else {
            "Pyodide not initialized!".to_string()
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
    py_wasm: PyWasmInterpreter,
}

#[godot_api]
impl NodeVirtual for PythonRunner {
    fn init(base: Base<Node>) -> Self {
        let mut py_wasm = PyWasmInterpreter::new();
        py_wasm.instantiate();
        py_wasm.install_pennylane(); // Install PennyLane when starting
        Self {
            base,
            py_wasm,
        }
    }
}

#[godot_api]
impl PythonRunner {
    #[func]
    fn run_python_script(&mut self, script: GString) -> GString {
        let result = self.py_wasm.run_python(script.to_string().as_str());
        GString::from(result)
    }
}
