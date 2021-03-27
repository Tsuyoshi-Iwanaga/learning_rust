fn main() {
  let _hoge1 :bool = true;
  let _hoge2 :char = 'A';
  let _hoge2_num :u8 = b'A';//ASKII文字は数字でも表現できる 65

  let _hoge3 :f32 = 0.01;
  let _hoge4 :f64 = 0.01;

  let _hoge5 :i8 = -100;
  let _hoge6 :i16 = -100;
  let _hoge7 :i32 = -100;
  let _hoge8 :i64 = -100;
  let _hoge9 :i128 = -100;
  let _hoge10 :isize = -100;

  let _hoge11 :u8 = 5_u8;//リテラルのデータ型を明示する
  let _hoge12 :u16 = 100;
  let _hoge13 :u32 = 100_000_000;
  let _hoge14 :u64 = 0x12;//16進数で18
  let _hoge15 :u128 = 0o22;//8進数で18
  let _hoge16 :usize = 0b010010;//10進数で18
}