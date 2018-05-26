//! Tree node implementation.

use super::{Tree,Forest,Iter,IterMut,SubtreeIter};
use rust::*;

pub struct Node<T> {
    pub(crate) down  : *mut Node<T>, // last child
    pub(crate) right : *mut Node<T>, // next sibling
    pub        data  : T,
}

impl<T> Node<T> {
    #[inline] pub(crate) fn set_child( &mut self, child: *mut Node<T> ) { self.down = child; }
    #[inline] pub(crate) fn reset_child( &mut self ) { self.set_child( null_mut() ); }
    #[inline] pub fn is_leaf( &self ) -> bool { self.down.is_null() }
    #[inline] pub(crate) unsafe fn has_only_one_child( &self ) -> bool { self.head() == self.tail() }

    #[inline] pub(crate) fn set_sib( &mut self, right: *mut Self ) { self.right = right; }
    #[inline] pub(crate) fn reset_sib( &mut self ) { self.right = self as *mut Self; }
    #[inline] pub(crate) fn has_no_sib( &self ) -> bool { self.right as *const Self == self as *const Self }

    #[inline] pub(crate) unsafe fn head( &self ) -> *mut Self { (*self.down).right }
    #[inline] pub(crate) fn tail( &self ) -> *mut Self { self.down }
    #[inline] pub(crate) unsafe fn new_head( &self ) -> *mut Node<T> { (*self.head()).right }

    #[inline] pub(crate) unsafe fn adopt( &mut self, child: *mut Node<T> ) { (*self.tail()).right = child; }

    /// Adds the tree as the first child.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::local::tr;
    /// let mut tree = tr(0) /tr(1);
    /// tree.push_front( tr(2) );
    /// assert_eq!( tree.to_string(), "0( 2 1 )" );
    /// ```
    #[inline] pub fn push_front( &mut self, mut tree: Tree<T> ) {
        unsafe {
            if self.is_leaf() {
                self.set_child( tree.root );
            } else {
                tree.set_sib( self.head() );
                self.adopt( tree.root );
            }
        }
        tree.clear();
    }

    /// add the tree as the last child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::local::tr;
    /// let mut tree = tr(0) /tr(1);
    /// tree.push_back( tr(2) );
    /// assert_eq!( tree.to_string(), "0( 1 2 )" );
    /// ```
    #[inline] pub fn push_back( &mut self, mut tree: Tree<T> ) {
        unsafe {
            if !self.is_leaf() {
                tree.set_sib( self.head() );
                self.adopt( tree.root );
            }
            self.set_child( tree.root );
        }
        tree.clear();
    }

    /// remove and return the first child
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::local::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// assert_eq!( tree.pop_front(), Some( tr(1) ));
    /// assert_eq!( tree.to_string(), "0( 2 )" );
    /// ```
    #[inline] pub fn pop_front( &mut self ) -> Option<Tree<T>> {
        if self.is_leaf() {
            None
        } else { unsafe {
            let front = self.head();
            if self.has_only_one_child() {
                self.reset_child();
            } else {
                (*self.tail()).right = self.new_head();
            }
            (*front).reset_sib();
            Some( Tree::from( front ))
        }}

    }

    /// add all the forest's trees at front of children list
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::local::tr;
    /// let mut tree = tr(0) /tr(1);
    /// tree.prepend( -tr(2)-tr(3) );
    /// assert_eq!( tree.to_string(), "0( 2 3 1 )" );
    /// ```
    #[inline] pub fn prepend( &mut self, mut forest: Forest<T> ) {
        if !forest.is_empty() {
            if self.is_leaf() {
                self.set_child( forest.tail() );
            } else { unsafe {
                let forest_head = forest.head();
                forest.set_sib( self.head() );
                self.adopt( forest_head );
            }}
            forest.clear();
        }
    }

    /// add all the forest's trees at back of children list
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::local::tr;
    /// let mut tree = tr(0) /tr(1);
    /// tree.append( -tr(2)-tr(3) );
    /// assert_eq!( tree.to_string(), "0( 1 2 3 )" );
    /// ```
    #[inline] pub fn append( &mut self, mut forest: Forest<T> ) {
        if !forest.is_empty() {
            if self.is_leaf() {
                self.set_child( forest.tail() );
                forest.clear();
            } else { unsafe {
                let forest_head = forest.head();
                forest.set_sib( self.head() );
                self.adopt( forest_head );
                self.set_child( forest.tail() );
            }}
            forest.clear();
        }
    }

    /// Provides a forward iterator over sub `Node`s
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::local::tr;
    /// let tree = tr(0) /tr(1)/tr(2);
    /// let mut iter = tree.children();
    /// assert_eq!( iter.next(), Some( tr(1).root() ));
    /// assert_eq!( iter.next(), Some( tr(2).root() ));
    /// assert_eq!( iter.next(), None );
    /// ```
    #[inline] pub fn children<'a>( &self ) -> Iter<'a,T> {
        if self.is_leaf() {
            Iter::new( null(), null() )
        } else { unsafe {
            Iter::new( self.head(), self.tail() )
        }}
    }

    /// Provides a forward iterator over sub `Node`s with mutable references.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::local::tr;
    /// let mut tree = tr(0) /tr(1)/tr(2);
    /// for child in tree.children_mut() { child.data *= 10; }
    /// assert_eq!( tree.to_string(), "0( 10 20 )" );
    /// ```
    #[inline] pub fn children_mut<'a>( &mut self ) -> IterMut<'a,T> {
        if self.is_leaf() {
            IterMut::new( null_mut(), null_mut() )
        } else { unsafe {
            IterMut::new( self.head(), self.tail() )
        }}
    }

    /// Provide an iterator over the tree `Node`'s subtrees for insert/remove at any position.
    /// See `Subtree`'s document for more.
    #[inline] pub fn subtrees<'a>( &mut self ) -> SubtreeIter<'a,T> {
        unsafe {
            if self.is_leaf() {
                SubtreeIter {
                    next: null_mut(), curr: null_mut(), prev: null_mut(), tail: null_mut(),
                    down : &mut self.down as *mut *mut Node<T>,
                    mark: PhantomData,
                }
            } else {
                SubtreeIter {
                    next : self.head(),
                    curr : null_mut(),
                    prev : self.down,
                    tail : self.down,
                    down : &mut self.down as *mut *mut Node<T>,
                    mark : PhantomData,
                }
            }
        }
    }
}

impl<T> Extend<Tree<T>> for Node<T> {
    fn extend<I:IntoIterator<Item=Tree<T>>>( &mut self, iter: I ) {
        for child in iter.into_iter() {
            self.push_back( child );
        }
    }
}

impl<T:Debug> Debug for Node<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        write!( f, "{:?} {:?}", self.data, self as *const Self )?;
        if !self.down.is_null() { write!( f, "_{:?}", self.down )?; }
        if self.right as *const Self != self as *const Self { write!( f, ">{:?}", self.right )?; }
        if !self.is_leaf() {
            write!( f, "( " )?;
            for child in self.children() {
                write!( f, "{:?} ", child )?;
            }
            write!( f, ")" )?;
        }
        write!( f, "" )
    }
}

impl<T:Display> Display for Node<T> {
    fn fmt( &self, f: &mut Formatter ) -> fmt::Result {
        if self.is_leaf() {
            self.data.fmt(f)
        } else {
            self.data.fmt(f)?;
            write!( f, "( " )?;
            for child in self.children() {
                write!( f, "{} ", child )?;
            }
            write!( f, ")" )
        }
    }
}

impl<T:PartialEq> PartialEq for Node<T> {
    fn eq( &self, other: &Self ) -> bool { self.data == other.data && self.children().eq( other.children() )}
    fn ne( &self, other: &Self ) -> bool { self.data != other.data || self.children().ne( other.children() )}
}

impl<T:Eq> Eq for Node<T> {}

impl<T:PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp( &self, other: &Self ) -> Option<Ordering> {
        match self.data.partial_cmp( &other.data ) {
            None          => None,
            Some( order ) => match order {
                Less    => Some( Less ),
                Greater => Some( Greater ),
                Equal   => self.children().partial_cmp( other.children() ),
            },
        }
    }
}

impl<T:Ord> Ord for Node<T> {
    #[inline] fn cmp( &self, other: &Self ) -> Ordering {
        match self.data.cmp( &other.data ) {
            Less    => Less,
            Greater => Greater,
            Equal   => self.children().cmp( other.children() ),
        }
    }
}

impl<T:Hash> Hash for Node<T> {
    fn hash<H:Hasher>( &self, state: &mut H ) {
        self.data.hash( state );
        for child in self.children() {
            child.hash( state );
        }
    }
}
