pub mod token;
use token::Token;
use token::TokenType;


pub struct Scanner {
    pub scanner:String,
}
impl Scanner{
    pub fn new (scanner:String)->Scanner{
        Scanner{
            scanner,
        }
    }
    pub fn scanear(&self,sc:&String) -> (Token,Token,Token,Token,){
        let mut _var:bool=true;
        let mut _cond:bool=true;
        let mut _exp_1:bool=true;
        let mut _exp_2:bool=true;
        let mut _is_str:bool=false;

        let mut _t_var:Token=Token::new(TokenType::Default,String::from(""));
        let mut _t_cond:Token=Token::new(TokenType::Default,String::from(""));
        let mut _t_exp_1:Token=Token::new(TokenType::Default,String::from(""));
        let mut _t_exp_2:Token=Token::new(TokenType::Default,String::from(""));

        let mut str_var=String::new();
        let mut str_cond=String::new();
        let mut str_exp_1=String::new();
        let mut str_exp_2=String::new();

        for i in sc.chars(){
            match i{
                '='=>{
                    if _var{
                        _var = false;
                        _t_var=Token::new(TokenType::Var,str_var.clone());
                    }else if _is_str{
                        if _cond{
                            str_cond.push(i);
                        }else if _exp_1{
                            str_exp_1.push(i);
                        }else if _exp_2{
                            str_exp_2.push(i);
                        }
                    }else if _cond{
                        str_cond.push(i);
                    }else{
                        println!("Sysntax error");
                        println!("Hint: <variable>=<Condition>?<Expression1>:<Expression2>");
                        break;
                        }

                },
                '?'=>{
                    if _cond&&!_is_str{
                        _cond=false;
                        _t_cond=Token::new(TokenType::Cond,str_cond.clone());
                    }else if _is_str{
                        if _cond{
                            str_cond.push(i);
                        }else if _exp_1{
                            str_exp_1.push(i);
                        }else if _exp_2{
                            str_exp_2.push(i);
                        }
                    }else {
                        println!("Sysntax error");
                        println!("Hint: <variable>=<Condition>?<Expression1>:<Expression2>");
                        break;
                    }
                },
                ':'=>{
                    if _exp_1&&!_is_str{
                        _exp_1=false;
                        _t_exp_1=Token::new(TokenType::Cond,str_exp_1.clone());
                    }else if _is_str{
                        if _cond{
                            str_cond.push(i);
                        }else if _exp_1{
                            str_exp_1.push(i);
                        }else if _exp_2{
                            str_exp_2.push(i);
                        }
                    }else {
                        println!("Sysntax error");
                        println!("Hint: <variable>=<Condition>?<Expression1>:<Expression2>");
                        break;
                    }
                },
                '"'=>{
                    if _var{
                        println!("Sysntax error");
                        println!("Hint: one variable can not contains '\"'");
                        break;
                    }else {
                        if _is_str{
                            _is_str=false;
                        }else{
                            _is_str=true;
                        }
                    }
                    if _cond{
                        str_cond.push(i);
                    } else if _exp_1{
                        str_exp_1.push(i);
                    } else if _exp_2{
                        str_exp_2.push(i);
                    }
                },
                '\n'=> {
                    _t_exp_2=Token::new(TokenType::Exp,str_exp_2.clone());
                    if _var{
                        _t_var=Token::new(TokenType::Var,str_var.clone());
                    }
                },
                _=>{
                    if _var{
                        str_var.push(i);
                    }else if _cond{
                        str_cond.push(i);
                    }else if _exp_1{
                        str_exp_1.push(i);
                    }else if _exp_2{
                        str_exp_2.push(i);
                    }
                }
            }
        }
    (_t_var,_t_cond,_t_exp_1,_t_exp_2)
    }
}