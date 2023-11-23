## 内容

1. expect() unwrap() unwrap_or() unwrap_or_default() 使用示例
2. `Option<T>` 上的常用方法示例

- .map()
- .cloned()
- .is_some()
- .is_none()
- .as_ref()
- .as_mut()
- .take()
- .replace()
- .and_then()

3. `Result<T, E>` 上的常用方法示例

- .map()
- .is_ok()
- .is_err()
- .as_ref()
- .as_mut()
- .and_then()
- .map_err()

4. `Option<T>` 和 `Result<T,E>`的互相转换

左边到右边  `ok_or()`，反过来 `ok()` 和 `err()`