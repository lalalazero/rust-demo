## iterator

- iter() 不可变
- iter_mut() 可变
- into_iter() 拿取所有权，原来的集合被消耗（消耗取决于原来的集合元素类型是否支持 clone，否则为 move）

`for item in c {}` 真面目，会自动调用迭代器 into_iter

```rust
let mut tmp_iter = c.into_iter();
while let Some(item) = tmp_iter.next() {}
```

`for in &c {}` 获取不可变引用（本质上是迭代器变成了 iter），`for in &mut c {}` 获取可变引用（本质上是迭代器变成了 iter_mut）
