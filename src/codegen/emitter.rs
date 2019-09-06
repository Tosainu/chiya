pub trait Emitter {
    fn emit_move_ptr(&mut self, offset: i32) -> String;
    fn emit_add(&mut self, n: i32) -> String;

    fn emit_call_putchar(&mut self) -> String;
    fn emit_call_getchar(&mut self) -> String;

    fn emit_loop_begin(&mut self) -> String;
    fn emit_loop_end(&mut self) -> String;

    fn emit_header(&self) -> String;
    fn emit_footer(&self) -> String;
}
