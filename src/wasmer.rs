use std::sync::Arc;

use rand::Rng;
use wasmer::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RustData(Arc<str>);

pub struct HostState {
    rust_data: Vec<RustData>,
    memory: Option<Memory>,
}

impl HostState {
    fn new(capacity: usize) -> Self {
        Self {
            rust_data: Vec::with_capacity(capacity),
            memory: None,
        }
    }
    fn set_memory(&mut self, memory: Memory) {
        self.memory = Some(memory);
    }

    fn get_memory(&self) -> &Memory {
        self.memory.as_ref().unwrap()
    }

    fn view<'a>(&'a self, store: &'a impl AsStoreRef) -> MemoryView<'a> {
        self.get_memory().view(store)
    }
}

pub fn sort_userdata(run: impl FnOnce(&mut dyn FnMut())) -> anyhow::Result<()> {
    #[cfg(feature = "wasmer_cranelift")]
    let engine: Engine = wasmer::sys::Cranelift::new().into();
    #[cfg(feature = "wasmer_singlepass")]
    let engine: Engine = wasmer::sys::Singlepass::new().into();
    #[cfg(feature = "wasmer_llvm")]
    let engine: Engine = wasmer::sys::LLVM::new().into();
    #[cfg(feature = "wasmer_wamr")]
    let engine: Engine = Default::default();

    let wasm = include_bytes!("../scripts/sort_userdata.wasm");
    let module = Module::new(&engine, wasm)?;

    type Env<'a> = FunctionEnvMut<'a, HostState>;
    let mut store = Store::new(engine);
    let env = FunctionEnv::new(&mut store, HostState::new(10_000));

    let import_object = imports! {
        "RustData" => {
            "new" => Function::new_typed_with_env(&mut store, &env, |mut env: Env, off: i32, len: i32| -> i32 {
                let id = env.data().rust_data.len() as i32;
                let view = env.data().view(&env);
                let slice = WasmSlice::<u8>::new(&view, off as u64, len as u64).unwrap();
                let bytes = slice.read_to_bytes().unwrap().freeze();
                let s = unsafe { core::str::from_utf8_unchecked(bytes.as_ref()) };
                env.data_mut().rust_data.push(RustData(s.into()));
                id
            }),
            "lt" => Function::new_typed_with_env(&mut store, &env, |env: Env, i: i32, j: i32| -> i32 {
                let data = &env.data().rust_data;
                (data[i as usize] < data[j as usize]) as i32
            }),
            "clear" => Function::new_typed_with_env(&mut store, &env, |mut env: Env| {
                env.data_mut().rust_data.clear();
            }),
            "rand" => Function::new_typed(&mut store, |n: i32| {
                rand::thread_rng().gen_range(0..n)
            }),
        },
    };

    let instance = Instance::new(&mut store, &module, &import_object)?;
    let memory = instance.exports.get_memory("memory")?;
    env.as_mut(&mut store).set_memory(memory.clone());

    let bench = instance
        .exports
        .get_typed_function::<(), ()>(&mut store, "bench")?;

    run(&mut || bench.call(&mut store).unwrap());

    Ok(())
}
