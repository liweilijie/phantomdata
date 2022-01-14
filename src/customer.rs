use std::{
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

/// 全局的一个原子 id 变量
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// 在这个例子里，Customer 有个额外的类型 T。
/// 通过类型 T，我们可以将用户分成不同的等级，比如免费用户是 Customer<FreePlan>、付费用户是 Customer<PersonalPlan>，
/// 免费用户可以转化成付费用户，解锁更多权益。
/// 使用 PhantomData 处理这样的状态，可以在编译期做状态的检测，避免运行期检测的负担和潜在的错误。
pub struct Customer<T> {
    id: u64,
    name: String,
    _type: PhantomData<T>,
}

/// 免费用户的功能
pub trait Free {
    fn feature1(&self);
    fn feature2(&self);
}

/// 付费用户高级功能
pub trait Personal: Free {
    fn advance_feature(&self);
}

impl<T> free for Customer<T> {
    fn feature1(&self) {
        println!("{} has feature 1", self.name);
    }

    fn feature2(&self) {
        println!("{} has feature 2", self.name);
    }
}

impl Free for Customer<PersonalPlan> {
    fn feature1(&self) {
        todo!()
    }

    fn feature2(&self) {
        todo!()
    }
}

impl Personal for Customer<PersonalPlan> {
    fn advance_feature(&self) {
        println!("Dear {}(as our valuable customer {}), enjoy this advanced feature!",
                 self.name, self.id);
    }
}

/// 免费用户
pub struct FreePlan;

/// 付费用户
pub struct PersonalPlan(f32);

impl<T> Customer<T> {
    pub fn new(name: String) -> Self {
        Self {
            id: NEXT_ID.fetch_and(1, Ordering::Relaxed),
            name,
            _type: PhantomData::default(),
        }
    }
}

impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(c: Customer<FreePlan>) -> Self {
        Self::new(c.name)
    }
}

/// 订阅成为付费用户
pub fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan> {
    let _plan = PersonalPlan(payment);
    // 存储 plan 到 DB
    // ...
    customer.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer() {
        // 一开始是个免费用户
        let customer = Customer::<FreePlan>::new("liwei".into());
        // 使用免费 feature
        customer.feature1();
        customer.feature2();

        // 用着用着觉得产品不错愿意付费了
        let customer = subscribe(customer, 6.99);
        customer.feature1();
        customer.feature2();
        // 付费用户解锁了新技能
        customer.advance_feature();
    }
}