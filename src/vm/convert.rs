// 将ast转换成指令操作集
use crate::base::ast::{Expression, ExpressionKind, StatementKind, StmtList};
use crate::base::func::{FuncManager, Function};
use crate::base::opcode::{InnerInstruction, Instruction, Operand};
use crate::parser::lexer::Location;
use std::collections::HashSet;
// use std::rc::Rc;

// 检查expression并将其放入列表指令中
macro_rules! check_expr_and_append {
    ($ex:expr, $v:expr, $idx:expr) => {
        if $ex.not_simple() {
            let res = parse_expr($ex, $idx);
            if res.is_err() {
                return Err(res.err().unwrap());
            }
            let mut tmp = res.ok().unwrap();
            $idx += tmp.len();
            $v.append(&mut tmp);
        }
    };
}

macro_rules! handle_expr_parse_err {
    ($ex:expr, $idx:expr) => {
        match parse_expr($ex, $idx) {
            Ok(t) => t,
            Err(e) => return Err(e),
        }
    };
}

macro_rules! instruction {
    ($l:expr, $ex:expr) => {
        Instruction {
            location: $l.clone(),
            node: $ex,
        }
    };
}

// 将语句转成指令集，并收集函数定义、结构体定义
#[allow(unused)]
pub fn parse_stmts(stmts: &StmtList, begin: usize) -> Result<Vec<Instruction>, String> {
    let mut list = vec![];
    let mut index;
    let mut variables = HashSet::new();

    for stmt in stmts {
        index = list.len() + begin;
        let location = &stmt.location;

        match &stmt.node {
            StatementKind::Break => {
                list.push(instruction!(location, InnerInstruction::Break));
            }

            StatementKind::Continue => {
                list.push(instruction!(location, InnerInstruction::Continue));
            }

            StatementKind::Return { value } => {
                check_expr_and_append!(value, list, index);
                // check_symbol_is_defined!(scope.clone(), &value);
                list.push(instruction!(
                    location,
                    InnerInstruction::Return {
                        value: value.to_operand(),
                    }
                ));
            }

            StatementKind::Assign { left, right } => {
                check_expr_and_append!(left, list, index);
                check_expr_and_append!(right, list, index);
                // check_symbol_is_defined!(scope.clone(), left);
                // check_symbol_is_defined!(scope.clone(), right);
                list.push(instruction!(
                    location,
                    InnerInstruction::Assign {
                        lhs: left.to_operand(),
                        rhs: right.to_operand(),
                    }
                ));
            }

            StatementKind::Move { left, right } => {
                check_expr_and_append!(left, list, index);
                check_expr_and_append!(right, list, index);
                // check_symbol_is_defined!(scope.clone(), left);
                // check_symbol_is_defined!(scope.clone(), right);
                list.push(instruction!(
                    location,
                    InnerInstruction::Move {
                        lhs: left.to_operand(),
                        rhs: right.to_operand(),
                    }
                ));
            }

            StatementKind::AugAssign { op, left, right } => {
                if left.not_simple() {
                    // check_symbol_is_defined!(scope.clone(), left);
                    let mut left_vec = handle_expr_parse_err!(left, index);
                    index += left_vec.len();
                    list.append(&mut left_vec);
                    list.push(instruction!(location, InnerInstruction::Repeat)); // 拷贝栈顶的值
                    index += 1;
                } else {
                    list.push(instruction!(
                        location,
                        InnerInstruction::Push {
                            value: left.to_operand(),
                        }
                    ));
                    index += 1;
                }
                // check_expr_and_append!(right, list, scope.clone(), index);
                if right.not_simple() {
                    // check_symbol_is_defined!(scope.clone(), right);
                    let mut right_vec = handle_expr_parse_err!(right, index);
                    // index += right_vec.len();
                    list.append(&mut right_vec);
                } else {
                    list.push(instruction!(
                        location,
                        InnerInstruction::Push {
                            value: right.to_operand(),
                        }
                    ));
                    // index += 1;
                }
                // check_symbol_is_defined!(scope.clone(), right);
                list.push(instruction!(
                    location,
                    InnerInstruction::Call {
                        value: Operand::Var(op.clone()),
                        num: 2,
                    }
                ));
                list.push(instruction!(
                    location,
                    InnerInstruction::Assign {
                        lhs: left.to_operand(),
                        rhs: Operand::Stack,
                    }
                ));
            }

            // ToDo
            StatementKind::VarDef {
                mutable,
                name,
                typ,
                assign,
            } => {
                // 检测在当前作用域是否已经定义该变量
                if variables.get(name).is_some() {
                    return Err(format!("Variable Error: {} has been defined.", name));
                }
                variables.insert(name);
                list.push(instruction!(
                    location,
                    InnerInstruction::DefVar {
                        mutable: *mutable,
                        name: Operand::Var(name.clone()),
                        typ: typ.clone(),
                    }
                ));
                index += 1;
                // 定义变量
                if let Some(expr) = assign {
                    check_expr_and_append!(expr, list, index);
                    // check_symbol_is_defined!(scope.clone(), expr);
                    list.push(instruction!(
                        location,
                        InnerInstruction::Assign {
                            lhs: Operand::Var(name.clone()),
                            rhs: expr.to_operand(),
                        }
                    ));
                }
            }

            // ToDo
            StatementKind::TypeDef { left, right } => {
                // let typeid = scope.borrow_mut().types.insert(right);
                // if let Some(id) = typeid {
                //     scope.borrow_mut().types.insert_alias(left, id);
                // } else {
                //     // 在当前作用域找不到的话，在不断在父作用域中寻找
                //     let mut p = scope.clone();
                //     loop {
                //         let p_scope = p.borrow().parent.clone();
                //         if let Some(tmp) = p_scope {
                //             println!("get scope {:?}", tmp.clone());
                //             if let Some(typ) = tmp.clone().borrow().types.get_item_by_key(left)
                //             {
                //                 println!("log1 {:?}", typ);
                //                 let id = scope.borrow_mut().types.insert_kind(typ);
                //                 scope.borrow_mut().types.insert_alias(left, id);
                //                 break;
                //             } else {
                //                 println!("log2 ");
                //                 p = tmp;
                //             }
                //         } else {
                //             return Err(format!(
                //                 "Type Error: {} is undefined. ({})",
                //                 right.to_string(),
                //                 stmt.location,
                //             ));
                //         }
                //     }
                // }
            }

            // ToDo
            // StatementKind::StructDef { name, value } => {
            //     // scope.borrow_mut().types.insert_struct(name, value.clone());
            //     StructManager::get_ins().register(name.clone(), value.clone());
            // }

            // 定义一个函数，将函数放到全局的管理中
            StatementKind::FuncDef {
                name,
                args,
                rettyp,
                block,
            } => {
                match parse_stmts(&block, 0) {
                    Ok(codes) => {
                        // 注册该函数
                        let id = FuncManager::get_ins().register(Function {
                            // name: name.clone(),
                            args: args.clone(),
                            rettyp: rettyp.clone(),
                            codes,
                        });
                        // 添加一行指令
                        list.push(instruction!(
                            location,
                            InnerInstruction::DefFunc {
                                name: name.clone(),
                                id,
                                sign: FuncManager::get_sign(args),
                            }
                        ));
                    }
                    Err(s) => return Err(s),
                };
            }

            StatementKind::Expression { expr } => {
                let mut tmp_vec = handle_expr_parse_err!(expr, index);
                list.append(&mut tmp_vec);
            }

            StatementKind::If { test, then, orelse } => {
                //   n: if $0 m+1  // else case
                //       ...
                //   m: goto x
                // m+1:  ...       // else statement
                //   x:  ...       // out of if statement

                check_expr_and_append!(test, list, index); // 插入test语句
                index += 1; // if语句的位置

                let mut then_vec = match parse_stmts(&then, index) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
                index += then_vec.len();

                // 根据有无else来获得跳转地址
                let if_addr = if orelse.len() > 0 { index + 1 } else { index };

                // 插入if
                list.push(instruction!(
                    location,
                    InnerInstruction::If {
                        value: test.to_operand(),
                        addr: if_addr,
                    }
                ));

                // 插入 then
                list.append(&mut then_vec);

                if orelse.len() > 0 {
                    index += 1; // 插入else的goto
                    let mut else_vec = match parse_stmts(&orelse, index) {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    };
                    index += else_vec.len();
                    // 在else语句前加一个goto用于跳出整个else语句块
                    list.push(instruction!(
                        location,
                        InnerInstruction::Goto { addr: index }
                    ));
                    list.append(&mut else_vec);
                };
            }

            StatementKind::While { test, then } => {
                //   a: test expr
                //   n: if $0 m+1
                //        ...
                //   m: goto a
                let beg_addr = index;

                check_expr_and_append!(test, list, index); // 插入test语句

                index += 1; // if语句的位置

                let mut then_vec = match parse_stmts(&then, index) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
                index += then_vec.len();

                // 插入IF
                list.push(instruction!(
                    location,
                    InnerInstruction::If {
                        value: test.to_operand(),
                        addr: index + 1, // 跳出整个圈
                    }
                ));

                // 替换掉出现的break和continue
                for i in then_vec.iter_mut() {
                    match i.node {
                        InnerInstruction::Break => {
                            *i = instruction!(location, InnerInstruction::Goto { addr: index + 1 })
                        }
                        InnerInstruction::Continue => {
                            *i = instruction!(location, InnerInstruction::Goto { addr: beg_addr })
                        }
                        _ => {}
                    }
                }

                // 插入then
                list.append(&mut then_vec);
                // 插入goto用来实现循环
                list.push(instruction!(
                    location,
                    InnerInstruction::Goto { addr: beg_addr }
                ));
            }

            // ToDo
            StatementKind::For { name, iter, then } => {}
        }
    }

    // 销毁在该块创建的变量
    for var in variables {
        list.push(instruction!(
            Location::default(),
            InnerInstruction::Destroy { value: var.clone() }
        ))
    }
    Ok(list)
}

// 将表达式转换成
#[allow(unused)]
fn parse_expr(expr: &Box<Expression>, begin: usize) -> Result<Vec<Instruction>, String> {
    let mut list = Vec::new();
    let mut index = begin;
    let location = &expr.location;

    match &expr.node {
        ExpressionKind::Liter { value } => list.push(instruction!(
            location,
            InnerInstruction::Push {
                value: value.to_operand(),
            }
        )),

        ExpressionKind::Ident { value } => list.push(instruction!(
            location,
            InnerInstruction::Push {
                value: Operand::Var(value.clone()),
            }
        )),

        ExpressionKind::Dot { left, right } => {
            check_expr_and_append!(left, list, index);
            check_expr_and_append!(right, list, index);
            // check_symbol_is_defined!(scope.clone(), left);
            list.push(instruction!(
                location,
                InnerInstruction::Dot {
                    lhs: left.to_operand(),
                    rhs: right.to_operand(),
                }
            ))
        }

        ExpressionKind::Call { callee, args } => {
            check_expr_and_append!(callee, list, index);
            for arg in args {
                let res = parse_expr(arg, index);
                if res.is_err() {
                    return Err(res.err().unwrap());
                }
                let mut arg_vec = res.ok().unwrap();
                index += arg_vec.len();
                list.append(&mut arg_vec);
            }
            list.push(instruction!(
                location,
                InnerInstruction::Call {
                    value: callee.to_operand(),
                    num: args.len(),
                }
            ))
        }

        ExpressionKind::And { left, right } => {
            check_expr_and_append!(left, list, index);
            index += 1; // if 语句本身占一条
            let mut tmp_vec = {
                let res = parse_expr(right, index);
                if res.is_err() {
                    return Err(res.err().unwrap());
                }
                res.ok().unwrap()
            };
            index += tmp_vec.len();
            // If
            list.push(instruction!(
                location,
                InnerInstruction::If {
                    value: left.to_operand(),
                    addr: index + 1,
                }
            ));
            // Then
            list.append(&mut tmp_vec);
            // GoTo
            list.push(instruction!(
                location,
                InnerInstruction::Goto { addr: index + 2 }
            ));
            // push false
            list.push(instruction!(
                location,
                InnerInstruction::Push {
                    value: Operand::False,
                }
            ));
        }

        ExpressionKind::Or { left, right } => {
            // check_symbol_is_defined!(scope.clone(), left);
            check_expr_and_append!(left, list, index);
            index += 1;
            // check_symbol_is_defined!(scope.clone(), right);
            let mut tmp_vec = handle_expr_parse_err!(right, index);
            index += tmp_vec.len();
            // Ifnot
            list.push(instruction!(
                location,
                InnerInstruction::IfNot {
                    value: left.to_operand(),
                    addr: index + 1,
                }
            ));
            // then
            list.append(&mut tmp_vec);
            // goto
            list.push(instruction!(
                location,
                InnerInstruction::Goto { addr: index + 2 }
            ));
            // push true
            list.push(instruction!(
                location,
                InnerInstruction::Push {
                    value: Operand::True,
                }
            ));
        }

        ExpressionKind::Not { expr } => {
            // check_symbol_is_defined!(scope.clone(), expr);
            check_expr_and_append!(expr, list, index);
            list.push(instruction!(
                location,
                InnerInstruction::Not {
                    value: expr.to_operand(),
                }
            ));
        }

        ExpressionKind::Question { test, then, orelse } => {
            check_expr_and_append!(test, list, index);

            index += 1; // 插入if
            let mut tmp_vec = handle_expr_parse_err!(then, index);
            index += tmp_vec.len(); // 插入then
            index += 1; // 插入Goto
            let if_addr = index;
            let mut else_vec = handle_expr_parse_err!(orelse, index);
            index += else_vec.len(); // 插入else

            list.push(instruction!(
                location,
                InnerInstruction::If {
                    value: test.to_operand(),
                    addr: if_addr,
                }
            ));
            list.append(&mut tmp_vec);
            list.push(instruction!(
                location,
                InnerInstruction::Goto { addr: index }
            ));
            list.append(&mut else_vec);
        }

        // 构造结构体字面量
        ExpressionKind::Struct { name, args } => {
            let mut new_args = args
                .iter()
                .map(|s| (s.0.clone(), &s.1))
                .collect::<Vec<(String, &Box<Expression>)>>();
            let num = new_args.len();
            new_args.sort_by(|a, b| a.0.cmp(&b.0));
            for arg in new_args {
                // check_symbol_is_defined!(scope.clone(), arg.1);
                let mut tmp_vec = handle_expr_parse_err!(arg.1, index);
                index += tmp_vec.len();
                list.append(&mut tmp_vec);
                index += 1;
                list.push(instruction!(
                    location,
                    InnerInstruction::Push {
                        value: arg.1.to_operand(),
                    }
                ));
            }
            list.push(instruction!(
                location,
                InnerInstruction::Struct {
                    value: name.clone(),
                    num,
                }
            ));
        } //
          // ExpressionKind::TypeMark {} => {}
    }
    Ok(list)
}
