use deno_core::{OpState, extension, op2};
use std::rc::Rc;

use crate::storage::ResultStorage;

/// Op: 存储 JavaScript 执行结果
///
/// 这个 op 允许 JavaScript 代码将执行结果存储到 Rust 端。
/// 使用 #[op2(fast)] 优化性能。
#[op2(fast)]
pub fn op_store_result(state: &mut OpState, #[string] value: String) {
    if let Some(storage) = state.try_borrow_mut::<Rc<ResultStorage>>() {
        storage.store(value);
    }
}

/// Op: 提前返回（用于Hook拦截）
///
/// 用于在JS执行过程中提前返回结果并终止执行。
/// 这在逆向工程中非常有用，例如Hook XMLHttpRequest.send拦截参数。
///
/// 实现方式：存储值到 ResultStorage 并标记为早期返回，
/// JavaScript 端需要抛出错误来中断执行
#[op2(fast)]
pub fn op_early_return(state: &mut OpState, #[string] value: String) {
    if let Some(storage) = state.try_borrow_mut::<Rc<ResultStorage>>() {
        storage.store(value.clone());
        storage.mark_early_return();
    }
}

// 扩展
// 注册自定义 ops 到 Deno Core runtime。
// storage 通过 options 传入，并在 state 初始化时设置。
extension!(
    pyexecjs_ext,
    ops = [op_store_result, op_early_return],
    options = {
        storage: Rc<ResultStorage>,
    },
    state = |state, options| {
        state.put(options.storage);
    }
);
