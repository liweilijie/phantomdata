// 在泛型函数中，有时候泛型参数可以非常复杂。比如泛型参数是一个闭包，闭包返回一个 Iterator，
// Iterator 中的 Item 又有某个约束。看下面的示例代码：
pub fn consume_iterator<F, Iter, T>(mut f: F)
where
    F: FnMut(i32) -> Iter, // F是一个闭包,接受i32, 返回Iter类型
    Iter: Iterator<Item = T>, // Iter是一个Iterator, Item是T类型
    T: std::fmt::Debug, // T是一个Debug trait
{
    // 根据F的类型, f(10)返回Iterator, 所以可以用for来循环
    for item in f(10) {
        println!("{:?}", item); // item 实现了Debug trait, 所以可以用 {:?} 来打印
    }
}

// 这个代码的泛型参数虽然非常复杂，不过一步步分解，其实并不难理解其实质：
// 1. 参数 F 是一个闭包，接受 i32，返回 Iter 类型；
// 2. 参数 Iter 是一个 Iterator，Item 是 T 类型；
// 3. 参数 T 是一个实现了 Debug trait 的类型。
//
// 这么分解下来，我们就可以看到，为何这段代码能够编译通过，同时也可以写出合适的测试示例，来测试它：

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_iterator() {
        // 不会panic或者出错
        consume_iterator(|i| (0..i).into_iter())
    }
}