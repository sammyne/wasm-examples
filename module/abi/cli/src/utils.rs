use core::slice;

use anyhow::Context;
use wasmtime::{AsContext, AsContextMut, Memory};

pub struct TinyMemory<'a, T>
where
    T: AsContext + AsContextMut,
{
    data: &'a Memory,
    ctx: T,
}

impl<'a, T> TinyMemory<'a, T>
where
    T: AsContext + AsContextMut,
{
    pub fn read<S>(&mut self, offset: usize) -> anyhow::Result<S>
    where
        S: Default + Sized,
    {
        let mut out = S::default();
        let data = unsafe {
            let data = &mut out as *mut S as *mut u8;
            slice::from_raw_parts_mut(data, std::mem::size_of::<S>())
        };

        self.data
            .read(&self.ctx, offset, data)
            .context("read greeting from memory")?;

        Ok(out)
    }

    pub fn write<S>(&mut self, offset: usize, data: &S) -> anyhow::Result<()>
    where
        S: Sized,
    {
        let data = unsafe {
            let data = data as *const S as *const u8;
            slice::from_raw_parts(data, std::mem::size_of::<S>())
        };

        self.data
            .write(&mut self.ctx, offset, data)
            .context("write greeting to memory")
    }

    pub fn new(data: &'a Memory, ctx: T) -> Self {
        Self { data, ctx }
    }
}
