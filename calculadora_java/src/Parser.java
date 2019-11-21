import java.util.Stack;
import java.util.*;

public class Parser {

  private String dato;

  public Parser(){
  }


  public double evaluarNumeros(String str)throws Exception{
    char entrada [] = str.toCharArray();
    String str_temp="";
    Stack <Character> pila_operadores = new Stack <Character>();
    Stack <String> salida = new Stack <String>();
    for (char i: entrada){
      switch (i){
        case '0': case '1':
        case '2': case '3':
        case '4': case '5':
        case '6': case '7':
        case '8': case '9':
        case '.':
            str_temp+=String.valueOf(i);
            break;
        case '(':
              if (!str_temp.equals("")) {
                  salida.push(str_temp);
                  str_temp=new String("");
              }
              pila_operadores.push(i);
              break;
        case ')':
            if (!str_temp.equals("")) {
              salida.push(str_temp);
              str_temp=new String("");
            }
            while (!pila_operadores.isEmpty()&&!String.valueOf(pila_operadores.peek()).equals("(")){
              salida.push(String.valueOf(pila_operadores.pop()));
            }

            if (String.valueOf(pila_operadores.peek()).equals("(")){
              pila_operadores.pop();
            }else{
              throw new Exception("Error");
            }

            break;
        case '+': case '-':
        case '*': case '/':
        case 'l': case 's':
            if (i=='-'&&str_temp.equals("")){
                  str_temp+="-";
            } else {
              if (!str_temp.equals("")){
                salida.push(str_temp);
                str_temp= new String("");
              }
              while (!pila_operadores.isEmpty()&&this.isHigherOrEqualPrecedence((char) pila_operadores.peek(),i)){
                salida.push(String.valueOf(pila_operadores.pop()));
              }
              pila_operadores.push(i);
            }
            break;
        case ' ': default:
            break;

      }

    }


    if (!str_temp.equals("")){
      salida.push(str_temp);
    }
    while (!pila_operadores.isEmpty()){
      salida.push(pila_operadores.pop().toString());
    }

    return evaluar(salida);
  }

  private double evaluar(Stack <String> posfix_exp){
    String op=posfix_exp.pop();

    double retorno=0;
    try{
      retorno = Double.parseDouble(op);
      return retorno;
    }catch(Exception e){
      if (op.equals("+")){
        return this.evaluar(posfix_exp)+evaluar(posfix_exp);
      } else if (op.equals("-")){
        double segundo = evaluar(posfix_exp);
        double primero = evaluar(posfix_exp);
        return primero-segundo;
      } else if (op.equals("*")){
        return this.evaluar(posfix_exp)*evaluar(posfix_exp);
      }
      else if (op.equals("/")){
        double denominador= evaluar(posfix_exp);
        double numerador = evaluar(posfix_exp);
        return numerador/denominador;
      } else if (op.equals("l")){
        return Math.log(this.evaluar(posfix_exp));
      }else if (op.equals("s")){
        return Math.sin(this.evaluar(posfix_exp));
      }
    }
    return 0.0;
  }


  public boolean evaluarCondiciones(String cond) throws Exception{
    String temp="",op="",op1="",op2="";

    for (char i : cond.toCharArray()){
        switch (i){
          case '>': case '<':
          case '!':
              if (op.length()==0){
                op1=temp;
                temp=new String("");
                op+=String.valueOf(i);
              }else {
                op= new String("");
                throw new Exception("Error");
              }
              break;
            case '=':
                if (op.length()==1||op.length()==0){
                  if (op.length()==0){
                    if (!temp.equals("")) {
                      op1=temp;
                      temp = new String();
                    }else{
                      throw new Exception("Error");
                    }
                  }
                  op+=String.valueOf(i);
                }else{
                  throw new Exception("Error");
                }
                break;
            case ' ':
                break;
            default:
                  temp+=String.valueOf(i);
                  break;
        }
    }
    if (!temp.equals("")){
        op2=temp;
    } else {
      throw new Exception("Error");
    }
    boolean retorno=false;
    //System.out.println("op1: "+op1+" op2: "+op2);
    if (op.equals("==")){


      retorno = this.evaluarNumeros(op1)==this.evaluarNumeros(op2);
    }else if (op.equals(">=")){

      retorno = this.evaluarNumeros(op1)>=this.evaluarNumeros(op2);
    } else if (op.equals("<=")){

      retorno = this.evaluarNumeros(op1)<=this.evaluarNumeros(op2);
    } else if (op.equals("!=")){

      retorno = this.evaluarNumeros(op1)!=this.evaluarNumeros(op2);
    } else if (op.equals("<")){

      retorno = this.evaluarNumeros(op1)<this.evaluarNumeros(op2);
    } else if (op.equals(">")){

      double primero = evaluarNumeros(op1);
      double segundo = evaluarNumeros(op2);
      retorno = primero>segundo;
    } else{
      throw new Exception("Error");
    }
    return retorno;
  }

  private boolean isHigherOrEqualPrecedence(char char1,char char2){
    short prec_c1,prec_c2;
    switch (char1){
      case '+': case'-':
        prec_c1=1;
        break;
      case '*': case '/':
        prec_c1=2;
        break;
      case 's': case 'l':
        prec_c1 = 3;
        break;
      default:
        prec_c1=0;
        break;
    }
    switch (char2) {
      case '+': case'-':
        prec_c2=1;
        break;
      case '*': case '/':
        prec_c2=2;
        break;
      case 's': case 'l':
        prec_c2 = 3;
        break;
      default:
        prec_c2=0;
        break;
    }
    return prec_c1>=prec_c2;
  }
}
