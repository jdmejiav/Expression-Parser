mod scanner;


use std::io::{
    self,
    Write
};

fn main() {
    loop {
        let mut entrada=String::new();
        print!(">>> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut entrada).unwrap();
        if entrada.trim()=="exit"{
            println!("Build finished");
            break;
        } else{
            let sc = scanner::Scanner::new(entrada);
            let (_var,_cond,_exp_1,_exp_2)=sc.scanear(&sc.scanner);

        }
    }
}
