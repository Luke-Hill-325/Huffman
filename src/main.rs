struct Node {
    data: Option<u8>,
    frequency: u32,
    code: u32,
    len: usize,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl PartialOrd for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other &Self) -> Option<Ordering> {
        self.frequency.partial_cmp(&other.frequency)
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other:&Self) -> bool {
        self.frequency.eq(&other.frequency)
    }
}

impl Node {
    fn new() -> Self {
        Self {
            data: None,
            frequency: 0,
            code: 0,
            len: 0,
            left: None,
            right: None,
        }
    }
}

fn create_tree(frequencies: &[u32; 32]) -> ([Rc<RefCell<Node>>; 32], Rc<RefCell<Node>>) {
    let mut leaves: Vec<Rc<RefCell<Node>>> = (0..32)
        .map(|_| Rc::new(RefCell::new(Node::new())))
        .collect::<Vec<_>>();
    let mut nodes = BinaryHeap::new();
    for (i, n_) in leaves.iter_mut.enumerate() {
        let mut n = ReffCell::borrow_mut(n_);
        n.data = Some(i as u8);
        n.frequency = frequencies[i as usize];
        drop(n);
        nodes.push(Reverse(n_.clone()));
    }
    while nodes.len() > 1 {
        let n1 = nodes.pop().unwrap().0;
        let n2 = nodes.pop().unwrap().0;
        let parent = Node {
            data: None,
            frequency: RefCell::borrow(&n1)
                .frequency
                .saturating_add(RefCell::borrow(&n2).frequency),
            code: 0,
            len: 0,
            left: Some(n1.clone()),
            right: Some(n2.clone()),
        };
        nodes.push(Reverse(Rc::new(RefCell::new(parent))))
    }

    let root = nodes.pop().unwrap().0;
    let mut queue = Vec::with_capacity(32);
    queue.push(root.clone());
    
    while let Some(n) = queue.pop() {
        let n = RefCell::borrow_mut(&n);
        if n.data.is_none() {
            let left = n.left.as_ref().unwrap_or_else(|| panic!());
            let right = n.right.as_ref().unwrap_or_else(|| panic!());
            {
                let mut left_r = RefCell::borrow_mut(left);
                let mut right_r = RefCell::borrow_mut(right);
                left_r.code = n.code << 1;
                left_r.len = n.len + 1;
                right_r.code = (n.code << 1) + 1;
                right_r.len = n.len + 1;
            }
            queue.push(right.clone());
            queue.push(left.clone());
        }
    }
    (leaves.try_into().unwrap_or_else(|_| panic!()), root)
}
fn main() {

}
