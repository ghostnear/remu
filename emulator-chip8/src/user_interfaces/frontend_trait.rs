pub trait Frontend
{
    fn has_quit(&self) -> bool;
    fn update(&mut self, emulator:&crate::Emulator, delta: f64);
    fn draw(&mut self, emulator: &mut crate::Emulator);
}