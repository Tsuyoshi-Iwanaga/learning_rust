//共用体
//メンバーが同じメモリを共有するデータ構造

union MyData {
  va: u16,
  vb: u32,
}

fn main() {
  let v = MyData {vb: 0};

  //共用体はCのコードを使うなどに限って使われる安全でないデータ構造のため
  //使用するためにはunsafeブロックで囲む必要がある
  unsafe {
    println!("v.va={}", v.va);
    println!("v.vb={}", v.vb);
  }
}