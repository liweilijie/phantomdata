# phantomdata

rust phantomdata type.

- 被广泛用在处理，数据结构定义过程中不需要，但是在实现过程中需要的泛型参数。
- **在定义数据结构时，对于额外的、暂时不需要的泛型参数，用 PhantomData 来“拥有”它们，这样可以规避编译器的报错。**

- phantomdata and generic


## identifier.rs

现在要设计一个 User 和 Product 数据结构，它们都有一个 u64 类型的 id。
然而我希望每个数据结构的 id 只能和同种类型的 id 比较，
也就是说如果 user.id 和 product.id 比较，编译器就能直接报错，拒绝这种行为。该怎么做呢？

