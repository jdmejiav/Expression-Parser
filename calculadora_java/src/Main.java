import java.util.Hashtable;
import java.util.Stack;
import java.util.Scanner;
import java.io.IOException;

public class Main {

        private Hashtable <String,Double> vars;
        private Stack <String> list_vars;
        public Main (){
          inicializar();
        }

        private void inicializar(){
          vars = new Hashtable<String,Double>();
          list_vars= new Stack<String>();
          list_vars.push("pi");
          list_vars.push("e");
          vars.put("pi",Math.PI);
          vars.put("e",Math.E);
          String entrada = "";
          Scanner sc = new Scanner(System.in);
          do {
            System.out.print(">>> ");
            entrada=sc.nextLine();
            try{
              Scan scan = new Scan(entrada);
              String splitPoints[] =scan.getSplitPoints();
              if (splitPoints[0]!=null&&splitPoints[1]==null){
                System.out.println(vars.get(splitPoints[0]));
              } else  {
                for (String var: list_vars){

                  if ((splitPoints[1]).contains(var)){
                    splitPoints[1]=(splitPoints[1]).replace(var,vars.get(var).toString());
                  }
                  if ((splitPoints[2]).contains(var)){
                    splitPoints[2]=(splitPoints[2]).replace(var,vars.get(var).toString());
                  }
                  if ((splitPoints[3]).contains(var)){
                    splitPoints[3]=(splitPoints[3]).replace(var,vars.get(var).toString());
                  }
                }
                splitPoints[1]=splitPoints[1].replace("log","l");
                splitPoints[2]=splitPoints[2].replace("log","l");
                splitPoints[3]=splitPoints[3].replace("log","l");
                splitPoints[1]=splitPoints[1].replace("sin","s");
                splitPoints[2]=splitPoints[2].replace("sin","s");
                splitPoints[3]=splitPoints[3].replace("sin","s");

                Parser parser = new Parser();
                double value;

                //System.out.println("cond "+splitPoints[1]);
                //System.out.println("exp1 "+splitPoints[2]);
                //System.out.println("exp2 "+splitPoints[3]);

                boolean cond = parser.evaluarCondiciones(splitPoints[1]);
                //System.out.println("cond result: "+cond);

                if (cond){
                  value=parser.evaluarNumeros(splitPoints[2]);
                } else {

                  value=parser.evaluarNumeros(splitPoints[3]);
                }
                list_vars.push (splitPoints[0]);
                vars.put(splitPoints[0],value);
                System.out.println(value);
              }
            }catch (Exception e){
              e.printStackTrace();
              continue;
            }

          }while (!entrada.trim().equals("exit"));
          System.out.println("Build finished");
        }


        public static void main (String []args){
            Main main = new Main ();
        }


}
