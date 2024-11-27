use std::cell::{RefCell};
use std::rc::{Rc};
use std::fmt::{Debug, Formatter, Result as FmtResult};

type Ptr<Value> = Rc<RefCell<ListNode<Value>>>;

#[derive(Clone)]
struct ListNode<Value: Debug + Clone> {
    value: Value,
    left: Option<Ptr<Value>>,
    right: Option<Ptr<Value>>
}

impl<Value: Debug + Clone> Drop for ListNode<Value> {
    fn drop(&mut self) {
        println!("drop {:?}", self.value);
    }
}

impl<Value: Debug + Clone> Debug for ListNode<Value> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "<{:?}> ", self.value)
    }
}

pub struct List<Value: Debug + Clone> {
    list: Option<Ptr<Value>>
}

impl<Value: Debug + Clone> Drop for List<Value> {
    fn drop(&mut self) {
        println!("drop {:?}", self);

        fn kill<Value: Debug + Clone>(item: &mut ListNode::<Value>, last: &Ptr<Value>) {
            match item.left {
                None => return,
                Some(_) => ()
            }

            let tmp = Rc::clone(item.left.as_ref().unwrap());

            item.left = None;
            item.right = None;

            if !Rc::ptr_eq(&tmp, last) {
                kill(&mut tmp.borrow_mut(), last);
            }
        }

        if let Some(ref head) = &self.list {
            kill(&mut head.as_ref().borrow_mut(), head);
        }
    }
}

impl<Value: Debug + Clone> Debug for List<Value> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self.list {
            None => write!(f, "List []"),
            Some(_) => {
                let p: &Ptr<Value> = &Rc::clone(self.list.as_ref().unwrap());

                fn dmp<Value: Clone + Debug>(p: &Ptr<Value>, last: &Ptr<Value>) -> String {
                    let b = p.borrow();
                    let ll = b.left.as_ref().unwrap().borrow();
                    let rr = b.right.as_ref().unwrap().borrow();

                    let left_back = ll.right.as_ref().unwrap();
                    let right_back = rr.left.as_ref().unwrap();

                    assert!(Rc::ptr_eq(left_back, right_back));
                    assert!(Rc::ptr_eq(p, left_back));
                    assert!(Rc::ptr_eq(p, right_back));

                    let sub = {
                        if !Rc::ptr_eq(p, last) {
                            dmp(p.borrow().right.as_ref().unwrap(), last)
                        }
                        else {
                            "".to_string()
                        }
                    };

                    format!("{}{:?}", sub, *p.borrow())
                }

                let cum = dmp(p, &Rc::clone(self.list.as_ref().unwrap().borrow().left.as_ref().unwrap()));

                write!(f, "List {}", cum)
            }
        }
    }
}


impl<Value: Debug + Clone> List<Value> {
    pub fn new() -> List<Value> {
        List::<Value> {
            list: None
        }
    }

    pub fn ins_first(&mut self, value: Value) {
        let c = ListNode::<Value> {
            value: value.clone(),
            left: None,
            right: None
        };

        let k = Rc::new(RefCell::new(c));

        let n = ListNode::<Value> {
            value,
            left: Some(Rc::clone(&k)),
            right: Some(Rc::clone(&k))
        };

        k.replace(n);

        self.list = Some(k)
    }

    pub fn ins_second(&mut self, value: Value) {
        let new_ptr = {
            let new_node = ListNode::<Value> {
                value: value,
                left: Some(Rc::clone(self.list.as_ref().unwrap())),
                right: Some(Rc::clone(self.list.as_ref().unwrap()))
            };

            Rc::new(RefCell::new(new_node))
        };

        let new_head = ListNode::<Value> {
            value: self.list.as_ref().unwrap().borrow().value.clone(),
            left: Some(Rc::clone(&new_ptr)),
            right: Some(Rc::clone(&new_ptr))
        };

        *self.list.as_ref().unwrap().borrow_mut() = new_head;
    }

    pub fn ins_next_sr(&mut self, value: Value) {
        let h_ptr: &Ptr::<Value> = self.list.as_ref().unwrap();

        let tmp = h_ptr.borrow();
        let n_ptr: &Ptr::<Value> = tmp.left.as_ref().unwrap();

        let mut next: ListNode<Value> = (*h_ptr.borrow()).clone();
        let mut head: ListNode<Value> = (*n_ptr.borrow()).clone();

        let t_ptr: Ptr::<Value> = {
            let this: ListNode<Value> = ListNode::<Value> {
                value,
                right: Some(Rc::clone(h_ptr)),
                left: Some(Rc::clone(n_ptr))
            };

            Rc::new(RefCell::new(this))
        };

        next.left = Some(Rc::clone(&t_ptr));
        head.right = Some(Rc::clone(&t_ptr));

        *h_ptr.borrow().left.as_ref().unwrap().borrow_mut() = head;

        drop(tmp);
        drop(t_ptr);

        *self.list.as_ref().unwrap().borrow_mut() = next;
    }

    pub fn ins_next_sl(&mut self, value: Value) {
        let h_ptr: &Ptr::<Value> = self.list.as_ref().unwrap();

        let tmp = h_ptr.borrow();
        let n_ptr: &Ptr::<Value> = tmp.right.as_ref().unwrap();

        let mut next: ListNode<Value> = (*h_ptr.borrow()).clone();
        let mut head: ListNode<Value> = (*n_ptr.borrow()).clone();

        let t_ptr: Ptr::<Value> = {
            let this: ListNode<Value> = ListNode::<Value> {
                value,
                left: Some(Rc::clone(h_ptr)),
                right: Some(Rc::clone(n_ptr))
            };

            Rc::new(RefCell::new(this))
        };

        next.right = Some(Rc::clone(&t_ptr));
        head.left = Some(Rc::clone(&t_ptr));

        *h_ptr.borrow().right.as_ref().unwrap().borrow_mut() = head;

        drop(tmp);
        drop(t_ptr);

        *self.list.as_ref().unwrap().borrow_mut() = next;
    }

    pub fn ins_right(&mut self, value: Value) {
        match &self.list {
            None => self.ins_first(value),
            Some(lst) => {
                if Rc::ptr_eq(lst, lst.borrow().right.as_ref().unwrap()) {
                    self.ins_second(value);
                    self.cycle_left();
                }
                else {
                    self.cycle_left();
                    self.ins_next_sr(value);
                    self.cycle_right();
                    //assert!(false)
                }

                //self.cycle_left();
            }
        };
    }

    pub fn ins_left(&mut self, value: Value) {
        match &self.list {
            None => self.ins_first(value),
            Some(lst) => {
                if Rc::ptr_eq(lst, lst.borrow().left.as_ref().unwrap()) {
                    self.ins_second(value);
                    self.cycle_right();
                }
                else {
                    self.cycle_left();
                    self.ins_next_sl(value);
                    self.cycle_right();
                }
            }
        };
    }

    pub fn del_right(&mut self) {
        match &self.list {
            None => (),
            Some(lst) => {
                if Rc::ptr_eq(lst, lst.borrow().right.as_ref().unwrap()) {
                }
                else {
                }
            }
        }
    }

    pub fn del_left(&mut self) {
        match &self.list {
            None => (),
            Some(lst) => {
                if Rc::ptr_eq(lst, lst.borrow().right.as_ref().unwrap()) {
                }
                else {
                }
            }
        }
    }

    pub fn cycle_left(&mut self) {
        let next = Some(Rc::clone(self.list.as_ref().unwrap().borrow().left.as_ref().unwrap()));
        self.list = next;
    }

    pub fn cycle_right(&mut self) {
        let next = Some(Rc::clone(self.list.as_ref().unwrap().borrow().right.as_ref().unwrap()));
        self.list = next;
    }
}

fn list_test() {
    let mut list: List<i64> = List::new();

    println!("{:?}", &list);

    list.ins_right(32);
    println!("* after 32");
    println!("{:?}", &list);

    {
        let n = list.list.as_ref().unwrap().as_ref().borrow();

        match n.left {
            None => panic!("wft #1"),
            Some(_) => ()
        };

        match n.right {
            None => panic!("wtf #2"),
            Some(_) => ()
        };

        assert!(Rc::ptr_eq(n.right.as_ref().unwrap(), n.left.as_ref().unwrap()));
        assert!(Rc::ptr_eq(list.list.as_ref().unwrap(), n.left.as_ref().unwrap()));
    }

    list.ins_right(64);
    println!("* after 64 L / R");
    println!("{:?}", &list);

    list.ins_right(42);
    println!("* after 42 R");
    println!("{:?}", &list);

    list.ins_right(13);
    println!("* after 13 R");
    println!("{:?}", &list);

    list.ins_right(6);
    println!("* after 6 R");
    println!("{:?}", &list);

    list.ins_left(-1);
    println!("* after -1 L");
    println!("{:?}", &list);

    list.ins_left(-2);
    println!("* after -2 L");
    println!("{:?}", &list);

    list.ins_left(-3);
    println!("* after -3 L");
    println!("{:?}", &list);
}

