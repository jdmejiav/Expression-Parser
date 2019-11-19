import java.util.Hashtable;
import java.util.Stack;
import java.util.Scanner;
import java.io.*;

public class Main {

        private Hashtable <String,Double> vars;
        private Stack <String> list_vars;
        public Main (){
          inicializar();
        }

        public static void main (String []args){
          Main main = new Main ();
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
              String splitPoints[]=scan.getSplitPoints();
              for (int i = 0;i<splitPoints.length;i++){
                System.out.print(splitPoints[i]);
              }
              System.out.println();
            }catch (Exception e){

            }

          }while (!entrada.trim().equals("exit"));
          System.out.println("Build finished");
        }

}
