import java.io.IOException;

public class Scan{

  private String splitPoints[];

  public Scan (String sc) throws Exception{
    splitPoints = new String[4];
    findTokens(sc);
  }

  private void findTokens(String sc) throws Exception{
    boolean var =true, cond = true,exp_1 =true, exp_2= true;
    boolean _is_str=false;
    StringBuilder str_var=new StringBuilder(),str_cond= new StringBuilder(),
    str_exp_1 = new StringBuilder(),str_exp_2=new StringBuilder();
    for (int i = 0;i< sc.length();i++){
      switch (sc.charAt(i)){
        case '=':
            if (var){
              var = false;
              this.splitPoints[0]=str_var.toString();
            }else if (_is_str){
              if (cond){
                str_cond.append(sc.charAt(i));
              }else if (exp_1){
                str_exp_1.append(sc.charAt(i));
              } else if (exp_2){
                str_exp_2.append(sc.charAt(i));
              }
            }else if (cond){
              str_cond.append(sc.charAt(i));
            } else {
              System.out.println("Sysntax error!");
              System.out.println("Hint: <variable>=<Condition>?<Expression1>:<Expression2>");
              throw new Exception("Variables can not contains '='");
            }
            break;
        case '?':
            if (cond&&!_is_str){
                cond=false;
                splitPoints[1]=str_cond.toString();
            }else if (_is_str){
                if (cond){
                  str_cond.append(sc.charAt(i));
                }else if (exp_1){
                  str_exp_1.append(sc.charAt(i));
                }else if (exp_2){
                  str_exp_2.append(sc.charAt(i));
                }
            } else {
              System.out.println("Sysntax error!");
              System.out.println("Hint: <variable>=<Condition>?<Expression1>:<Expression2>");
              throw new Exception("Not recognized token '?'");
            }
            break;
          case ':':
              if (exp_1&&!_is_str){
                exp_1=false;
                splitPoints[2]=str_exp_1.toString();
              }else if (_is_str){
                  if (cond){
                      str_cond.append(sc.charAt(i));
                  }else if (exp_1){
                      str_exp_1.append(sc.charAt(i));
                  }else if (exp_2){
                      str_exp_2.append(sc.charAt(i));
                  }
              } else {
                System.out.println("Sysntax error!");
                System.out.println("Hint: <variable>=<Condition>?<Expression1>:<Expression2>");
                throw new Exception("Not recognized token ':'");
              }
              break;
          case '"':
            if (var){
              System.out.println("Sysntax error!");
              System.out.println("Hint: one variable can not contains '\"'");
              throw new Exception("Not recognized token ':'");
            }else{
              if (_is_str){
                _is_str=false;
              }else{
                _is_str=true;
              }
            }
            if (cond){
              str_cond.append(sc.charAt(i));
            }else if (exp_1){
              str_exp_1.append(sc.charAt(i));
            }else if (exp_2){
              str_exp_2.append(sc.charAt(i));
            }
            break;
          case '\n':
              break;
          case ',':
              if (var){
                str_var.append(".");
              }else if (cond){
                  str_cond.append(".");
              }else if (exp_1){
                  str_exp_1.append(".");
              }else if (exp_2){
                  str_exp_2.append(".");
              }
              break;

          default:
              if (var){
                str_var.append(sc.charAt(i));
              }else if (cond){
                  str_cond.append(sc.charAt(i));
              }else if (exp_1){
                  str_exp_1.append(sc.charAt(i));
              }else if (exp_2){
                  str_exp_2.append(sc.charAt(i));
              }
              break;
      }
    }

    if (var){
      splitPoints[0]=str_var.toString();
    }else{
      splitPoints[3]=str_exp_2.toString();
    }
  }
  public String[] getSplitPoints(){
    return splitPoints;
  }
}
