pub struct Token{
    pub tipo:TokenType,
    pub data:String,
}
impl Token {
    pub fn new (tipo:TokenType,data:String) -> Token{
        Token{
            tipo,
            data,
        }
    }
    pub fn analizar (&mut self){

    }

}

pub enum TokenType{
    Cond,
    Exp,
    Var,
    Default,
}

pub struct Parser {
    token:Token,
    _is_string_op:bool,
}
impl Parser{
    pub fn new (token:Token)->Parser{
        let _is_string_op = Parser::identify_op(&token.data);
        Parser {
            token,
            _is_string_op,
        }
    }
    fn identify_op(str:&String) -> bool{
        str.contains("\"")
    }

    pub fn parsear_cond (&self){
        if self._is_string_op{
            let mut operadores:Vec<String> = Vec::new();
            let mut operandos:Vec<String> = Vec::new();
            let mut _is_str:bool=false;
            let mut str_temp = String::new();
            let mut str_op_temp = String::new ();
            for i in self.token.data.chars(){
                match i{
                    ' '=>{
                        if _is_str{
                            str_temp.push(i);
                        }
                    },
                    '\"'=>{
                        if _is_str{
                            _is_str=false;
                        }else{
                            _is_str=true;
                            if str_op_temp.len()!=0{
                                operadores.push(str_op_temp.clone());
                            }
                        }
                    },
                    '='=>{
                        if _is_str{
                            str_temp.push(i);
                        } else {
                            str_op_temp.push(i);
                            if str_op_temp.len()==2{
                                operadores.push(str_op_temp.clone());
                            }else {
                                if str_op_temp.len()==1{
                                    if str_temp.len()!=0{
                                        operandos.push(str_temp.clone());
                                        str_temp.clear();
                                    }else {
                                        println!("Sysntax Error!");
                                        break;
                                    }
                                }else if str_op_temp.len()>2{
                                    println!("Sysntax Error!");
                                    break;
                                }
                            }
                        }
                    },
                    '>'| '<' =>{
                        if _is_str{
                            str_temp.push(i);
                        } else {
                            str_op_temp.push (i);
                            if str_op_temp.len()>1{
                                println!("Sysntax Error!");
                                break;
                            }else if str_op_temp.len()==1{
                                if str_temp.len()!=0{
                                    operandos.push(str_temp.clone());
                                }else {
                                    println!("Sysntax Error!");
                                    break;
                                }
                            }

                        }

                    },
                    '!'=>{
                        if _is_str{
                            str_temp.push(i);
                        } else {
                            str_op_temp.push (i);
                            if str_op_temp.len()>1{
                                println!("Sysntax Error!");
                                break;
                            }else if str_op_temp.len()==1{
                                if str_temp.len()!=0{
                                    operandos.push(str_temp.clone());
                                }else {
                                    println!("Sysntax Error!");
                                    break;
                                }
                            }

                        }

                    },
                    _=>{
                        str_temp.push(i);
                    },
                }
            }
            if (str_temp.len()!=0){
                operandos.push(str_temp);
            }
        }else{

        }
    }
    fn evaluar_operaciones_int(mut expression:String)->isize{
        return 1;
    }

    fn evaluar_operaciones_float(mut expression:String)->f64{
        expression=expression.trim().to_string();
        expression=expression.replace(",",".");
        let mut numeros: Vec<f64> = Vec::new();
        let mut operadores: Vec<String> = Vec::new();
        let mut temp = String::new ();
        let temp
        for i in expression.chars(){
            match i{
                '+' | '-' =>{
                    let temporal=i.to_string();
                    operadores=instert_at_bottom_str(operadores,temporal);
                    numeros=instert_at_bottom_f64(numeros,temp);
                    temp.clear();
                },
                '*' | '/' =>{
                    operadores.push(i.to_string());
                    numeros.push(temp.parse().ok().expect("Error"));
                    temp.clear();
                },
                's'|'i'|'n' =>{

                },
                'l'|'o'|'g' =>{

                }
                _=>{
                    temp.push(i);
                }
            };
        }
        return 1.0;
    }

}
 fn instert_at_bottom_str(mut temp_vec: Vec<String>,var:String)->Vec<String>{
     let mut vec = vec![var];
     vec.append(&mut temp_vec);
     vec
 }
 fn instert_at_bottom_f64(mut temp_vec: Vec<f64>,var:f64)->Vec<f64>{
     let mut vec = vec![var];
     vec.append(&mut temp_vec);
     vec
 }
