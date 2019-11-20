import math

def evaluarNumeros(str):
    salida = []
    pila_operadores = []
    str_temp=""
    for i in str:
        if i.isdigit() or i=='.':
            str_temp+=i
        elif i=='(':
                if str_temp!="":
                    salida.append(str_temp)
                    str_temp=""
                pila_operadores.append(i)
        elif i == ')':
            if str_temp!="":
                salida.append(str_temp)
                str_temp=""
            while len(pila_operadores)!=0 and pila_operadores[-1]!="(":
                salida.append(pila_operadores.pop())

            if pila_operadores[-1]=='(':
                pila_operadores.pop()
            else:
                print('Sysntax Error')

        elif i=='+' or i=='-' or i=='*' or i=='/' or i=='l' or i=='s':
            if i=='-' and str_temp=="":
                str_temp+=i
            else:
                if str_temp!="":
                    salida.append(str_temp)
                    str_temp=""
                while len(pila_operadores)!=0 and isHigherOrEqualPrecedence(pila_operadores[-1],i):
                    salida.append(pila_operadores.pop())
                pila_operadores.append(i)

    if str_temp!="":
        salida.append(str_temp)
    while len(pila_operadores):
        salida.append(pila_operadores.pop())
    return evaluar(salida)

def evaluar(posfix_exp):
    op = posfix_exp.pop()

    try:
        return float(op)
    except:
        if op=='+':
            return evaluar(posfix_exp)+evaluar(posfix_exp)
        elif op=='-':
            segundo = evaluar(posfix_exp)
            primero = evaluar(posfix_exp)
            return primero-segundo
        elif op=='*':
            return evaluar(posfix_exp)*evaluar(posfix_exp)
        elif op=='/':
            denominador = evaluar(posfix_exp)
            numerador = evaluar(posfix_exp)
            return numerador/denominador
        elif op=='l':
            return math.log(evaluar(posfix_exp))
        elif op=='s':
            return math.sin(evaluar(posfix_exp))

def evaluarCondiciones(cond):
    temp =""
    op=""
    op1=""
    op2=""

    for i in cond:
        if i=='>' or i=='<' or i=='!':
            if len(op)==0:
                op1=temp
                temp=""
                op+=i
            else:
                print('Syntax Error!')
                break;
        elif i=='=':
            if len(op)==1 or len(op)==0:
                if len(op)==0:
                    if temp!="":
                        op1=temp
                        temp=""
                    else:
                        print('Syntax Error!')
                        break;
                op+=i
            else:
                print('Syntax Error!')
        elif i!=' ':
            temp+=i

    if temp!="":
        op2=temp
    else:
        print('Sysntax Error!')

    if op=='==':
        return evaluarNumeros(op1)==evaluarNumeros(op2)
    elif op=='>=':
        return evaluarNumeros(op1)>=evaluarNumeros(op2)
    elif op=='<=':
        return evaluarNumeros(op1)<=evaluarNumeros(op2)
    elif op=='!=':
        return evaluarNumeros(op1)!=evaluarNumeros(op2)
    elif op=='>':
        return evaluarNumeros(op1)>evaluarNumeros(op2)
    elif op=='<':
        return evaluarNumeros(op1)<evaluarNumeros(op2)




def isHigherOrEqualPrecedence(char1,char2):
    prec_c1=0
    prec_c2=0

    if char1=='+' or char1=='-':
        prec_c1=1
    elif char1=='*' or char1=='/':
        prec_c1=2
    elif char1=='l' or char1=='s':
        prec_c1=3

    if char2=='+' or char2=='-':
        prec_c2=1
    elif char2=='*' or char2=='/':
        prec_c2=2
    elif char2=='l' or char2=='s':
        prec_c2=3

    return prec_c1>prec_c2
