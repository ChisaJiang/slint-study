fn main() {
    // 编译ui/main.slint，路径对应上面目录
    slint_build::compile("ui/todo.slint").unwrap();
}
