use std::collections::VecDeque;

use crate::codegen::emitter;

pub struct LLVM {
    variable_idx: u32,
    label_idx: u32,
    loop_stack: VecDeque<u32>,
}

impl LLVM {
    pub fn new() -> LLVM {
        Default::default()
    }
}

impl Default for LLVM {
    fn default() -> Self {
        LLVM {
            variable_idx: 1,
            label_idx: 1,
            loop_stack: VecDeque::new(),
        }
    }
}

impl emitter::Emitter for LLVM {
    fn emit_move_ptr(&mut self, offset: i32) -> String {
        let s = format!(
            r#"
  ; emit_move_ptr({2})
  %{0} = load i32*, i32** %ptr, align 8
  %{1} = getelementptr inbounds i32, i32* %{0}, i32 {2}
  store i32* %{1}, i32** %ptr, align 8"#,
            self.variable_idx,
            self.variable_idx + 1,
            offset
        );
        self.variable_idx += 2;

        s
    }

    fn emit_add(&mut self, n: i32) -> String {
        let s = format!(
            r#"
  ; emit_add({3})
  %{0} = load i32*, i32** %ptr, align 8
  %{1} = load i32, i32* %{0}, align 4
  %{2} = add nsw i32 %{1}, {3}
  store i32 %{2}, i32* %{0}, align 4"#,
            self.variable_idx,
            self.variable_idx + 1,
            self.variable_idx + 2,
            n
        );
        self.variable_idx += 3;

        s
    }

    fn emit_call_putchar(&mut self) -> String {
        let s = format!(
            r#"
  ; emit_call_putchar()
  %{0} = load i32*, i32** %ptr, align 8
  %{1} = load i32, i32* %{0}, align 4
  %{2} = call i32 @putchar(i32 %{1})"#,
            self.variable_idx,
            self.variable_idx + 1,
            self.variable_idx + 2,
        );
        self.variable_idx += 3;

        s
    }

    fn emit_call_getchar(&mut self) -> String {
        let s = format!(
            r#"
  ; emit_call_getchar()
  %{0} = load i32*, i32** %ptr, align 8
  %{1} = call i32 @getchar()
  store i32 %{1}, i32* %{0}, align 4"#,
            self.variable_idx,
            self.variable_idx + 1,
        );
        self.variable_idx += 2;

        s
    }

    fn emit_loop_begin(&mut self) -> String {
        let s = format!(
            r#"
  ; emit_loop_begin()
  br label %loop{3}_cond
loop{3}_cond:
  %{0} = load i32*, i32** %ptr, align 8
  %{1} = load i32, i32* %{0}, align 4
  %{2} = icmp ne i32 %{1}, 0
  br i1 %{2}, label %loop{3}_body, label %loop{3}_end
loop{3}_body:"#,
            self.variable_idx,
            self.variable_idx + 1,
            self.variable_idx + 2,
            self.label_idx
        );
        self.loop_stack.push_back(self.label_idx);
        self.variable_idx += 3;
        self.label_idx += 1;

        s
    }

    fn emit_loop_end(&mut self) -> String {
        self.loop_stack
            .pop_back()
            .map(|n| {
                format!(
                    r#"
  ; emit_loop_end()
  br label %loop{0}_cond
loop{0}_end:"#,
                    n
                )
            })
            .unwrap_or_else(|| "".to_owned())
    }

    fn emit_header(&self) -> String {
        r#"; emit_header()
define i32 @main() {
  %heap_i8 = call i8* @calloc(i64 30000, i64 4)
  %heap_i32 = bitcast i8* %heap_i8 to i32*
  %heap = alloca i32*, align 8
  %ptr = alloca i32*, align 8
  store i32* %heap_i32, i32** %heap, align 8
  store i32* %heap_i32, i32** %ptr, align 8"#
            .to_owned()
    }

    fn emit_footer(&self) -> String {
        format!(
            r#"
  ; emit_footer()
  %{0} = load i32*, i32** %heap, align 8

  %{1} = bitcast i32* %{0} to i8*
  call void @free(i8* %{1})
  ret i32 0
}}

declare i8* @calloc(i64, i64)
declare void @free(i8*)
declare i32 @getchar()
declare i32 @putchar(i32)"#,
            self.variable_idx,
            self.variable_idx + 1
        )
    }
}
