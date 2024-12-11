pub struct List<I> {
    pub head: ListLink<I>,
}
pub struct ListNode<I> {
    pub item: I,
    pub next: ListLink<I>,
}
pub type ListLink<I> = Option<Box<ListNode<I>>>;

pub struct Tree<I> {
    pub root: TreeLink<I>,
    pub degree: usize,
}
pub struct TreeNode<I> {
    pub item: I,
    pub child: TreeLink<I>,
    pub sibling: TreeLink<I>,
}
pub type TreeLink<I> = Option<Box<TreeNode<I>>>;

impl<I> Default for List<I> {
    fn default() -> Self {
        Self { head: None }
    }
}
impl<I> Default for Tree<I> {
    fn default() -> Self {
        Self {
            root: None,
            degree: 0,
        }
    }
}

impl<I> Drop for List<I> {
    fn drop(&mut self) {
        let mut curr = self.head.take();
        while let Some(mut boxed) = curr {
            curr = boxed.next.take();
        }
    }
}
impl<I> Drop for Tree<I> {
    fn drop(&mut self) {
        let mut curr_vert = self.root.take();
        while let Some(mut boxed_vert) = curr_vert {
            let mut curr_hor = boxed_vert.sibling.take();
            while let Some(mut boxed_hor) = curr_hor {
                curr_hor = boxed_hor.sibling.take();
            }
            curr_vert = boxed_vert.child.take();
        }
    }
}

impl<I> List<I> {
    pub fn with_item(item: I) -> Self {
        Self {
            head: Some(Box::new(ListNode { item, next: None })),
        }
    }
}
impl<I> Tree<I> {
    pub fn with_item(item: I) -> Self {
        Self {
            root: Some(Box::new(TreeNode {
                item,
                child: None,
                sibling: None,
            })),
            degree: 0,
        }
    }
}

pub struct IntoIterList<I>(ListLink<I>);
impl<I> Iterator for IntoIterList<I> {
    type Item = Box<ListNode<I>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|mut x| {
            *self = Self(x.next.take());
            x
        })
    }
}
impl<I> IntoIterator for List<I> {
    type Item = Box<ListNode<I>>;

    type IntoIter = IntoIterList<I>;

    fn into_iter(mut self) -> Self::IntoIter {
        IntoIterList(self.head.take())
    }
}
