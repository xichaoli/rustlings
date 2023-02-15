// modules1.rs
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a hint.

// 默认情况下，模块中的项拥有私有的可见性（private visibility）
// 项可以是：函数，结构体，trait，impl 块，甚至其它模块。

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
