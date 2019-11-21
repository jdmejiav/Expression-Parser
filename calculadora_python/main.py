import math
import scanner
import parser

vars={}

def main():
    while True:
        vars["e"]=math.e
        vars["pi"]=math.pi
        entrada = input(">>> ")
        if entrada=="exit":
            break;
        else:
            _var,_cond,_exp_1,_exp_2=scanner.findTokens(entrada)
            if not _var in vars:
                for i in vars:
                    if i in _cond:
                        _cond=_cond.replace(i,str(vars[i]))
                    if i in _exp_1:
                        _exp_1=_exp_1.replace(i,str(vars[i]))
                    if i in _exp_2:
                        _exp_2=_exp_2.replace(i,str(vars[i]))

                _cond=_cond.replace("log","l")
                _exp_1=_exp_1.replace("log","l")
                _exp_2=_exp_2.replace("log","l")

                _cond=_cond.replace("sin","s")
                _exp_1=_exp_1.replace("sin","s")
                _exp_2=_exp_2.replace("sin","s")
                value = 0.0
                try:
                    cond = parser.evaluarCondiciones(_cond)
                    if cond:
                        value=parser.evaluarNumeros(_exp_1)
                    else:
                        value=parser.evaluarNumeros(_exp_2)

                    vars[_var]=value
                    print(value)
                except:
                    print('Something went wrong')
                    continue
            else:
                print(vars[_var])



if __name__=="__main__":
    main()
    print("build finished")
