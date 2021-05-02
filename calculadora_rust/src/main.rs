pub mod scanner;

use scanner::Scanner;
use scanner::token::Parser;
use std::collections::HashMap;

use std::io::{
    self,
    Write
};

fn main() {
    let mut vars: HashMap<String,f64>= HashMap::new();
    let mut _list_vars:Vec<String>=Vec::new();
    _list_vars.push("pi".to_string());
    _list_vars.push("e".to_string());
    vars.insert("pi".to_string(),std::f64::consts::PI);
    vars.insert("e".to_string(),std::f64::consts::E);
    loop {
        let mut entrada=String::new();
        print!(">>> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut entrada).unwrap();
        if entrada.trim()=="exit"{
            println!("Build finished");
            break;
        } else{
            let sc = Scanner::new(entrada);
            let (mut _var,mut _cond,mut _exp_1,mut _exp_2)=sc.scanear(&sc.scanner);
            let vacio = String::new();
            for i in _list_vars.iter(){
                let _value_temp:&str = &vars.get(&i.clone()).unwrap().to_string();
                if (&_cond).data.contains(&i.clone()) {
                    _cond.data=_cond.data.replace(&i.clone(),_value_temp.clone());
                }
                if (&_exp_1).data.contains(&i.clone()) {
                    _exp_1.data=_exp_1.data.replace(&i.clone(),_value_temp.clone());
                }
                if (&_exp_2).data.contains(&i.clone()) {
                    _exp_2.data=_exp_2.data.replace(&i.clone(),_value_temp.clone());
                }
            }

            _cond.data=_cond.data.replace(",",".");
            _exp_1.data=_exp_1.data.replace(",",".");
            _exp_2.data=_exp_2.data.replace(",",".");
            _cond.data=_cond.data.trim().to_string();
            _exp_1.data=_exp_1.data.trim().to_string();
            _exp_2.data=_exp_2.data.trim().to_string();


            if _var.data!=vacio&&_cond.data==vacio&&_exp_1.data==vacio&&_exp_2.data==vacio{

                match vars.get(&_var.data){
                    Some (p)=>{
                        println!("{}",p);
                    },
                    None=>{
                        println!("Variable {}, not found",&_var.data);
                    }
                }
            }else {
                let mut _cond_temp= _cond.data;

                _cond_temp=_cond_temp.replace("log","l");
                _cond_temp=_cond_temp.replace("sin","s");

                _exp_1.data=_exp_1.data.replace("log","l");
                _exp_2.data=_exp_2.data.replace("log","l");
                _exp_1.data=_exp_1.data.replace("sin","s");
                _exp_2.data=_exp_2.data.replace("sin","s");
                let cond_exp:bool = Parser::evaluar_condiciones(&mut _cond_temp);
                let exp_value:f64;
                 if cond_exp{
                     let mut exp1=_exp_1.data;
                     exp_value=Parser::evaluar_numeros(&mut exp1);
                 }else{
                     let mut exp2=_exp_2.data;
                     exp_value=Parser::evaluar_numeros(&mut exp2);
                 }
                 vars.insert(_var.data.trim().to_string().clone(),exp_value);
                 _list_vars.push(_var.data);
                 println!("{}",exp_value);
            }

        }
    }
}
