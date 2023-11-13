use re_exporting_example::kinds::PrimaryColor; // 没有 re-export 时需要这样导入
use re_exporting_example::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}