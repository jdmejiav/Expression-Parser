
use std::collections::HashMap;

pub trait Node{
    //pub fn add (&self,dato:String);
}
pub enum Tipe {
    Add,
    Subs,
    Mult,
    Div,
    Log,
    Sin,
    Num,
}

pub struct Num {
    pub tipo:Tipe,
    pub num:f64,
}
impl Node for Num {
    /*fn add(&self,dato:String)->Option<Node>{
       let temp:f64= parse().ok.expect("No es un npumero");
       Some(Num{
           Tipe::Num,
           temp,
       })
   }*/
}
pub struct Suma{
    tipo:Tipe,
    left: Box<Node>,
    rigth: Box<Node>,
}


impl Node for Suma{
    /*pub fn add(&self,dato:String){
        match self.left{
            Some (s)=>{
                match s.rigth.{

                }
            }
        }
    }*/
}
pub struct Resta{
    tipo:Tipe,
    left: Box<Node>,
    rigth: Box<Node>,
}

impl Node for Resta{


}
pub struct Multiplicacion{
    tipo:Tipe,
    left: Box<Node>,
    rigth: Box<Node>,
}

impl Node for Multiplicacion{

}
pub struct Division{
    tipo:Tipe,
    left: Box<Node>,
    rigth: Box<Node>,
}

impl Node for Division{

}
pub struct Seno{
    tipo:Tipe,
    child: Box<Node>,
}

impl Node for Seno{

}
pub struct Ln{
    tipo:Tipe,
    child: Box<Node>,
}

impl Node for Ln{

}

pub struct MayorQue{
    left:Box<Node>,
    rigth:Box<Node>
}
impl Node for MayorQue{


}

pub struct MenorQue{
    left:Box<Node>,
    rigth:Box<Node>
}

impl Node for MenorQue{

}

pub struct Igual{
    left:Box<Node>,
    rigth:Box<Node>
}
impl Node for Igual{

}
pub struct MayorIgual{
    left:Box<Node>,
    rigth:Box<Node>
}
impl Node for MayorIgual{

}
pub struct MenorIgual{
    left:Box<Node>,
    rigth:Box<Node>
}
impl Node for MenorIgual{

}
