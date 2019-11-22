


def findTokens(sc):
    _var=""
    _cond=""
    _exp_1=""
    _exp_2=""
    var=True
    cond=True
    exp_1=True
    exp_2=True
    _is_str=False
    str_var=""
    str_cond=""
    str_exp_1=""
    str_exp_2=""
    for i in sc:
        if i=='=':
            if var:
                var=False
                _var=str_var
            elif _is_str:
                if cond:
                    str_cond+=i
                elif exp_1:
                    str_exp_1+=i
                elif exp_2:
                    str_exp_2+=i
            elif cond:
                str_cond+=i
            else:
                print('Sysntax error!')
                print('Hint: <variable>=<Condition>?<Expression1>:<Expression2>')
                print('Variables can not contains =')
                break;
        elif i=='?':
            if cond and not _is_str:
                cond=False
                _cond=str_cond
            elif _is_str:
                if cond:
                    str_cond+=i
                elif exp_1:
                    str_exp_1+=i
                elif exp_2:
                    str_exp_2+=i
            else:
                print('Sysntax error!')
                print('Hint: <variable>=<Condition>?<Expression1>:<Expression2>')
                print('Not recognized token ?')
                break;
        elif i==':':
            if exp_1 and not _is_str:
                exp_1=False
                _exp_1=str_exp_1
            elif _is_str:
                if cond:
                    str_cond+=i
                elif exp_1:
                    str_exp_1+=i
                elif exp_2:
                    str_exp_2+=i
            else:
                print('Sysntax error!')
                print('Hint: <variable>=<Condition>?<Expression1>:<Expression2>')
                print('Not recognized token :')
                break;
        elif i=='"':
            if var:
                print('Sysntax error!')
                print('Hint: one variable can not contains "')
                break;
            else:
                if _is_str:
                    _is_str=False
                else:
                    _is_str=True
            if cond:
                str_cond+=i
            elif exp_1:
                str_exp_1+=i
            elif exp_2:
                str_exp_2+=i
        elif i==',':
            if var:
                str_var+='.'
            elif cond:
                str_cond+='.'
            elif exp_1:
                str_exp_1+='.'
            elif exp_2:
                str_exp_2+='.'
        else:
            if var:
                str_var+=i
            elif cond:
                str_cond+=i
            elif exp_1:
                str_exp_1+=i
            elif exp_2:
                str_exp_2+=i
    if var:
        _var=str_var
    else:
        _exp_2=str_exp_2

    return _var,_cond,_exp_1,_exp_2
