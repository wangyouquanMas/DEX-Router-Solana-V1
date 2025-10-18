目标：
1. 掌握load_mut()方法


内容：
1. load_mut()的作用？

In Solana Anchor programs, load_mut() is a method used with AccountLoader to obtain
a mutable reference to the underlying account data,

举例：
let mut my_account = ctx.accounts.my_account.load_mut()?;

This is typically used when you need to write to an account that is zero-copy enabled. 


类比： 独占 + 直接可修改
Think of the account’s bytes as a shared notebook locked in a cabinet.

load() is like opening the glass cover and reading the notebook. You don’t take it out or write in it—others can still come by and look too.

load_mut() is like taking the notebook out with the only editing pen. You’re writing directly on the original pages (zero-copy). While you have it, nobody else can read or edit that notebook. If someone tries, they’ll be told “it’s checked out.”
When you’re done (the RefMut goes out of scope or you drop it), you put it back and others can access it again.

Key bits mapped:

Notebook = the account’s data

Taking it out with the pen = load_mut() returning RefMut<T>

Writing on original pages = in-place mutation (no extra buffer)

“Checked out” sign = borrow error if you try another load/load_mut before returning it