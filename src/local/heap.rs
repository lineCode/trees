//! Dynamic allocation/deallocation on heap.

use super::Node;

use rust::*;

pub type Phantom<T> = PhantomData<Box<Node<T>>>;

pub(crate) fn make_node<T>( data: T ) -> *mut Node<T> {
    let mut node = Box::new( Node {
        down  : null_mut(),
        right : null_mut(),
        data  : data,
    });
    node.reset_sib();
    Box::into_raw( node )
}

pub(crate) fn drop_node<T>( node: *mut Node<T> ) { unsafe{ Box::from_raw( node ); }}
