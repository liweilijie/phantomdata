# phantomdata

rust phantomdata type.

- 被广泛用在处理，数据结构定义过程中不需要，但是在实现过程中需要的泛型参数。
- **在定义数据结构时，对于额外的、暂时不需要的泛型参数，用 PhantomData 来“拥有”它们，这样可以规避编译器的报错。**

- phantomdata and generic

## BufReader
我们以标准库的`BufReader`结构为例，先简单回顾一下，在定义数据结构和实现数据结构时，如果使用了泛型参数，到底有什么样的好处。
```rust
pub struct BufReader<R> {
    inner: R,
    buf: Box<[u8]>,
    pos: usize,
    cap: usize,
}
```
`BufReader`对要读取的`R`做了一个泛型的抽象。也就是说，`R`此刻是个`File`，还是一个`Cursor`，或者直接是`Vec<u8>`，都不重要。
在定义`struct`的时候，我们并未对`R`做进一步的限制，这是最常用的使用泛型的方式。延迟到后面实现的时候再去做限制.


## identifier.rs

现在要设计一个 User 和 Product 数据结构，它们都有一个 u64 类型的 id。
然而我希望每个数据结构的 id 只能和同种类型的 id 比较，
也就是说如果 user.id 和 product.id 比较，编译器就能直接报错，拒绝这种行为。该怎么做呢？

# customer.rs

在这个例子里，Customer 有个额外的类型 T。
通过类型 T，我们可以将用户分成不同的等级，比如免费用户是 Customer<FreePlan>、付费用户是 Customer<PersonalPlan>，
免费用户可以转化成付费用户，解锁更多权益。
使用 PhantomData 处理这样的状态，可以在编译期做状态的检测，避免运行期检测的负担和潜在的错误。

## function.rs

Rust 目前还不支持在 trait 里使用 impl trait 做返回值.
那怎么办？很简单，我们可以返回 trait object，它消除了类型的差异，把所有不同的实现 Iterator 的类型都统一到一个相同的 trait object 下：

## complex_args.rs
复制的泛型,一步一步分解不乱就能理解 