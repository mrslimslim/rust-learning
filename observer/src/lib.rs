/*
 * @Description:
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-05-29 11:32:41
 * @LastEditors: tengyu
 * @LastEditTime: 2022-05-29 11:38:42
 */
trait IObserver {
    // fn update(&self, subject: &Subject);
    fn update(&self);
}

trait ISubject<'a, T: IObserver> {
    fn attach(&mut self, observer: &'a T);
    fn detach(&mut self, observer: &'a T);
    fn notify_observers(&mut self);
}

struct Subject<'a, T: IObserver> {
    observers: Vec<&'a T>,
}

impl<'a, T: IObserver + PartialEq> Subject<'a, T> {}
