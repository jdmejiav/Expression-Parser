
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
}

pub enum TokenType{
    Cond,
    Exp,
    Var,
    Default,
}

pub struct Parser {
}
impl Parser{
    pub fn evaluar_numeros (string: &mut String)->f64{
        let mut revert:Vec<char>=string.chars().collect();
            let mut salida:Vec<String> = Vec::new();
            let mut entrada:Vec<char>=Vec::new();
            let mut pila:Vec<String>=Vec::new();
            let mut str_temp = String::new();

            while !revert.is_empty(){
                match revert.pop(){
                    Some (c) => entrada.push(Some(c).unwrap()),
                    None => None.unwrap()
                }
            }
            while !entrada.is_empty(){
                let i = match entrada.pop(){
                    Some(c) => (Some(c).unwrap()),
                    None => None.unwrap()
                };
                match i {
                    '0' | '1' | '2' | '3'
                    | '4' | '5' | '6' |
                    '7' | '8' | '9' =>{
                        str_temp.push(i);
                    },
                    '('=>{
                        if str_temp!=""{
                            salida.push(str_temp);
                            str_temp= String::new();
                        }
                        pila.push(i.to_string());

                        },
                    ')'=>{
                        if str_temp!=""{
                            salida.push(str_temp);
                            str_temp= String::new();
                        }
                      while !pila.is_empty()&&string_to_char(&mut pila.clone().pop().unwrap())!='('  {
                          let temp = pila.pop().unwrap();
                          salida.push (temp);
                      }
                      if string_to_char(&mut pila.clone().pop().unwrap())=='('{
                          pila.pop();
                      }else {
                          println!("Sysntax Error!");
                          break;
                      }
                    },
                    '+'|'-'
                    |'*'|'/'|
                    's'|'l'=>{
                       if str_temp!=""{
                            salida.push(str_temp);
                            str_temp= String::new();
                        }
                       while !pila.is_empty()&&is_higher_or_equal_precedence(string_to_char(&mut pila.clone().pop().unwrap()),i){

                           let temp = pila.pop().unwrap();
                           salida.push(temp);
                       }
                       pila.push(i.to_string());

                    },
                    ' '=>{},
                    _=>{}
                }
            }

            if str_temp!=""{
                salida.push(str_temp);
            }
            while !pila.is_empty(){
                let temp:String = pila.pop().unwrap();
                salida.push(temp);
            }


        return Parser::evaluar(&mut salida);
    }

    pub fn evaluar(posfix_exp :&mut Vec<String>)->f64{

        let retorno:f64= match posfix_exp.clone().pop().unwrap().parse(){
            Ok(num)=>{
                posfix_exp.pop();
                num
            },
            Err(_)=>{
                let op:String = posfix_exp.pop().unwrap();
                if op == "+"{
                    let mut temp = posfix_exp.clone();
                    temp.pop();

                    return Parser::evaluar(&mut temp)+Parser::evaluar(&mut posfix_exp.clone());
                }else if op == "-"{
                    let mut temp = posfix_exp.clone();
                    temp.pop();

                    return Parser::evaluar(&mut temp)-Parser::evaluar(&mut posfix_exp.clone());
                }else if op == "*"{

                    let mut temp = posfix_exp.clone();
                    temp.pop();
                    return Parser::evaluar(&mut temp)*Parser::evaluar(&mut posfix_exp.clone());
                }else if op == "/"{
                    let mut temp = posfix_exp.clone();
                    temp.pop();

                    return Parser::evaluar(&mut temp)/Parser::evaluar(&mut posfix_exp.clone());
                }else if op == "l"{
                    return Parser::evaluar(&mut posfix_exp.clone()).ln();
                }else if op == "s"{
                    return Parser::evaluar(&mut posfix_exp.clone()).sin();
                }else{
                    -1.0
                }
            }
        };
        retorno
    }


    pub fn evaluar_condiciones(cond:&mut String) ->bool {
        let mut temp = String::new ();

        let mut op=String::new();
        let mut op1=String::new ();
        let mut op2=String::new ();

        for i in cond.chars(){

            match i {
                '>'|'<'|'!'=>{
                    if op.len()==0{
                        if temp!=String::new(){
                            op1=temp.clone();
                            temp=String::new();
                            op.push(i);
                        } else {
                            op=String::from("ERROR");
                            break;
                        }
                    }
                },
                '=' =>{
                    if op.len()==1||op.len()==0 {
                        if op.len()==0{
                            if temp!=String::new(){
                                op1=temp.clone();
                                temp=String::new();
                            }else{
                                op=String::from("ERROR");
                                break;
                            }
                        }
                        op.push(i);
                    }else{
                        op=String::from("ERROR");
                        break;
                    }
                },
                ' '=>{

                },
                _=> {
                    temp.push(i);

                }
            }
        }
        if temp!=String::new(){
            op2=temp.clone();
        }else{
            op=String::from("ERROR");
        }
        let mut retorno=false;
        if op=="=="{
            retorno = Parser::evaluar_numeros(&mut op1)==Parser::evaluar_numeros(&mut op2);
        }else if op==">="{
            retorno = Parser::evaluar_numeros(&mut op1)>=Parser::evaluar_numeros(&mut op2);
        }else if op=="<="{
            retorno = Parser::evaluar_numeros(&mut op1)<=Parser::evaluar_numeros(&mut op2);

        }else if op=="!="{
            retorno = Parser::evaluar_numeros(&mut op1)!=Parser::evaluar_numeros(&mut op2);
        } else if op=="<"{
            retorno = Parser::evaluar_numeros(&mut op1)<Parser::evaluar_numeros(&mut op2);
        }else if op==">"{
            retorno =Parser::evaluar_numeros(&mut op1)>Parser::evaluar_numeros(&mut op2);
        }else if op=="ERROR"{
            println!("Sysntax Error!");
            {
                panic!("");
            }
        }
        retorno
    }

}

fn is_higher_or_equal_precedence(char1:char,char2:char)->bool{
     let prec_c1= match char1{
         '+'|'-'=>{
             1
         }
         '*'|'/' =>{
             2
         }
         's'|'l' =>{
             3
         }
         _=>(0)
     };
     let prec_c2= match char2{
         '+'|'-'=>{
             1
         }
         '*'|'/' =>{
             2
         }
         's'|'l' =>{
             3
         },
         _=>{
             0
         }
     };
     prec_c1>=prec_c2
 }

 fn string_to_char(string:&mut String)->char{
    string.chars().next().unwrap()
 }
