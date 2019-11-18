mod ast;

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

    fn to_posfix_exp(&self) -> Vec<String>{
        let mut salida: Vec<String> = Vec::new();
        let mut revert: Vec<char> = self.token.data.chars().collect();
        let mut entrada:Vec<char>= Vec::new();
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
            println!("I:{}",i);
            match i {
                '0' | '1' | '2' | '3'
                | '4' | '5' | '6' |
                '7' | '8' | '9' =>{
                    println!("llega acá");
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
                       println!("entra acá por {}",i);
                       let temp = pila.pop().unwrap();
                       salida.push(temp);
                   }
                   pila.push(i.to_string());

                },
                '\n'=>{
                    if str_temp!=""{
                        salida.push(str_temp);
                        str_temp= String::new();
                    }
                }
                _=>{
                    println!("Sysntax Error!");
                    break;
                }
            }
        }
        while !pila.is_empty(){
            let temp:String = pila.pop().unwrap();
            salida.push(temp);
        }

        return salida;

    }

    fn to_ast(&self,posfix_exp:Vec<String>){
        let mut _numeros:Vec<f64>=Vec::new();

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
        for i in expression.chars(){
            match i{
                '+' | '-' =>{
                    let temporal=i.to_string();
                    operadores=instert_at_bottom_str(operadores,temporal);
                    numeros=instert_at_bottom_f64(numeros,temp.parse().ok().expect("Error"));
                    temp=String::new();
                },
                '*' | '/' =>{
                    operadores.push(i.to_string());
                    numeros.push(temp.parse().ok().expect("Error"));
                    temp = String::new();
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

fn insert_at_begin(vec:&mut Vec<f64>,temp:f64)->Vec<f64>{
    let mut retorno:Vec<f64> = Vec::new();
    retorno.push(temp);
    retorno.append(vec);
    retorno
}
