#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


mod module_a;
// pub useを使ってmodule_aのcalcを外部から参照できるようにする
pub use module_a::calc;


mod module_b {
    fn calc() {
        println!("外部から呼べないmodだよ");
    }
}

mod module_a_out;
mod module_b_out;