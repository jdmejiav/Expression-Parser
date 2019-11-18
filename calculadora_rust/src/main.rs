pub mod scanner;
use std::collections::HashMap;

use std::io::{
    self,
    Write
};

fn main() {
    let mut vars: HashMap<String,f64>= HashMap::new();
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
            let vacio = String::new();
            if _var.data!=vacio&&_cond.data==vacio&&_exp_1.data==vacio&&_exp_2.data==vacio
            {
                match vars.get(&_var.data){
                    Some (p)=>{
                        println!("{}",p);
                    },
                    None=>{
                        println!("Variable {}, not found",&_var.data);
                    }
                }
            }

        }
    }
}
