# Expression Parser

## Syntax:

       <var>=<conditionn>?<Math_exp_1>:<Math_exp_2>
       

       
##  Valid math operator:
        Binarios = +,-,*,/
        unarios = - , log, sin
        paréntesis = (,)
 
##  Constants by default:
<ul>
       <li>
              pi=3.141592... 
       </li>  
       <li> 
              e=2.7182818...
       </li>
       
</ul>

Usage mode: 

       log(pi+e) 
       sin(pi/2) 
       e + pi * 4
       e + pi + <Previously defined variable>

Note: Logarithm and sin expression, must be between parents, ex: log(4+5*7) + sin(4*8)

# Calculadora Rust:
       
       Se ejecuta con cargo utlizando el comando cargo run, dentro del paquete calculadora_rust
       o dentro de la carperta ./calculadora_rust/run/ se encuentra el archivo ejecutable.out, que es un ejecutable de la calculadora
       o bien también utilizando el compilador de rust (rustc), compilando el archivo main.rs, ubicado en ./calculadora_rust/src/ para
       generar un nuevo ejecutable

# calculadora Java:
 
       Se ejecuta conrriendo el archivo run_file.sh en la carpeta ./calculadora_java/src/
       en la carpeta ./calculadora_java/run/ ya se encuentra el archivo Main.class que es la última versión del archivo compilado

# calculadora Python:
 
       Se ejecuta corriendo el archivo main.py localizado en el paquete calculadora_python con el intérprete de  python (python3) 
       python ./calculadora_python/main.py       

